# ===== Stage 1: Build =====
FROM rust:1.84.1 as builder

# ตั้งค่า working directory
WORKDIR /usr/src/myapp

# คัดลอกไฟล์ Cargo.toml และ Cargo.lock ก่อน (เพื่อแคช dependencies)
COPY Cargo.toml Cargo.lock ./

# สร้าง dummy source file เพื่อแคช dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# ดึง dependencies และคอมไพล์เบื้องต้นเพื่อแคช
RUN cargo build --release

# คัดลอกโค้ดทั้งหมด
COPY . .

# คอมไพล์โปรเจกต์จริงใน Release mode
RUN cargo build --release

# ===== Stage 2: Production =====
FROM debian:bookworm-slim

# ติดตั้ง dependencies ที่จำเป็น (ถ้าแอปต้องการ dynamic libraries)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# ตั้งค่า working directory สำหรับ Production
WORKDIR /usr/local/bin

# คัดลอกเฉพาะไบนารีจาก Stage 1
COPY --from=builder /usr/src/myapp/target/release/myapp ./myapp

# รันไบนารีเมื่อ container เริ่มทำงาน
CMD ["./myapp"]