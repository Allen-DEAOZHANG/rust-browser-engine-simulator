# Rust-Based Browser Engine Simulator

This project is a simplified browser engine written in Rust that simulates core rendering logic including DOM tree construction, layout computation, and style resolution.

It is inspired by modern browser architectures such as Servo and Chromium, and serves as an educational exploration of low-latency UI rendering infrastructure.

---

## 📌 Features

- Parses basic HTML input into a DOM tree
- Computes block-level layout recursively
- Resolves inline styles and applies cascading rules
- Uses Rust's thread-safe structures to simulate async DOM updates

---

## 🧠 Architecture

```
[HTML Input]
      ↓
[DOM Parser]
      ↓
[Style Resolver] → [Layout Engine] → [Box Tree Output]
```

---

## 🔧 Technologies

- Rust 2021 Edition
- Multi-threaded concurrency (`std::thread`, `Arc`, `Mutex`)
- Custom HTML tokenizer and style matcher

---

## 🚀 How to Run

> Requires Rust installed (https://rustup.rs)

```bash
cargo build --release
cargo run --release
```

---

## 📄 Sample HTML

```
<div style="width:100;height:50;background:red;">
  <p style="font-size:16px;">Hello World</p>
</div>
```

---

## 🖼 Output

```text
Layout Box Tree:
Div(width=100, height=50, background=red)
└── Paragraph(font-size=16px): Hello World
```

---

## 🧑‍💻 Author

[Deao Zhang](https://linkedin.com/in/deao-zhang-87993b249)
