# Actix Web 4.3.0 Boilerplate + Prisma Client Rust 0.6.4
* set environment variables `DATABASE_URL` and `SHADOW_DATABASE_URL` in development.env
* `cargo migrate` - Migrate schema to database and generate Rust code at `/src/app/db/prisma.rs`
* `cargo generate` - Generate schema to Rust code at `/src/app/db/prisma.rs` without migrating to database
