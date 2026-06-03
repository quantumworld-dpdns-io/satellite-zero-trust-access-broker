package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/redis/go-redis/v9"
)

var ctx = context.Background()
var redisClient *redis.Client

func initRedis() {
	redisClient = redis.NewClient(&redis.Options{
		Addr:     "localhost:6379",
		Password: "", // no password set
		DB:       0,  // use default DB
	})

	// Check connection
	_, err := redisClient.Ping(ctx).Result()
	if err != nil {
		log.Printf("Warning: Failed to connect to Redis. JIT caching will be disabled. Error: %v", err)
	} else {
		log.Println("Successfully connected to Redis for JIT Token caching.")
	}
}

func authMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		token := r.Header.Get("X-JIT-Token")
		if token == "" {
			http.Error(w, "Forbidden: Missing JIT Token", http.StatusForbidden)
			return
		}

		// Verify token against Redis cache
		val, err := redisClient.Get(ctx, "jit_token:"+token).Result()
		if err == redis.Nil {
			http.Error(w, "Forbidden: Invalid or Expired JIT Token", http.StatusForbidden)
			return
		} else if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		log.Printf("Access granted for user: %s", val)
		next.ServeHTTP(w, r)
	}
}

func telemetryHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Telemetry endpoint reached securely.")
}

func generateJitTokenHandler(w http.ResponseWriter, r *http.Request) {
	// In reality, this would require hardware MFA verification via the Rust Enclave first
	token := "dummy-quantum-safe-token-12345"
	user := "operator-alpha"

	err := redisClient.Set(ctx, "jit_token:"+token, user, 5*time.Minute).Err()
	if err != nil {
		http.Error(w, "Failed to generate token", http.StatusInternalServerError)
		return
	}

	fmt.Fprintf(w, "JIT Token generated: %s (Expires in 5m)", token)
}

func main() {
	initRedis()
	fmt.Println("Satellite Zero Trust Access Broker - Gateway Starting...")

	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "OK")
	})

	http.HandleFunc("/auth/token", generateJitTokenHandler)
	http.HandleFunc("/api/telemetry", authMiddleware(telemetryHandler))

	log.Println("Listening on :8080")
	// log.Fatal(http.ListenAndServe(":8080", nil))
}
