# Use the official Rust image with the latest version
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the rest of the source code
COPY . .

EXPOSE 6060

Run ls -trlah

# Run the application
CMD ["./web"]
