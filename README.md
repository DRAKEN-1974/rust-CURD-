# 🚀 Actix Web CRUD with Supabase

This project is a **Rust backend API** built with [Actix Web](https://actix.rs/) that performs **CRUD (Create, Read, Update, Delete)** operations using [Supabase](https://supabase.com/) as the database.

---

## 📌 Features
- Create new records in Supabase
- Read (fetch) all records
- Update records
- Delete records
- JSON-based API
- `.env` file support for sensitive credentials

---

## 🛠 Tech Stack
- **Rust** (Backend language)
- **Actix Web** (Web framework)
- **Supabase** (PostgreSQL database + REST API)
- **serde** (Serialization/Deserialization)
- **dotenv** (Environment variable handling)
- **reqwest** (HTTP client for Supabase API calls)

---

## 📂 Project Structure
.
├── src
│ ├── main.rs # Application entry point
│ ├── handlers.rs # CRUD operation handlers
│ ├── models.rs # Data structures
│ └── utils.rs # Helper functions
├── .env # Environment variables
├── Cargo.toml # Rust dependencies
└── README.md # Project documentation

yaml
Copy
Edit

---

## ⚙️ Setup & Installation

### 1️⃣ Clone the Repository
```bash
git clone https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git
cd YOUR_REPO_NAME
2️⃣ Install Rust
If you don’t have Rust installed:

bash
Copy
Edit
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
3️⃣ Create a .env File
Inside the project root, create a .env file:

env
Copy
Edit
SUPABASE_URL=https://your-supabase-url.supabase.co
SUPABASE_KEY=your-supabase-service-role-key
SUPABASE_TABLE=your_table_name
⚠️ Never commit your .env file to GitHub! Add it to .gitignore.

4️⃣ Install Dependencies
bash
Copy
Edit
cargo build
5️⃣ Run the Server
bash
Copy
Edit
cargo run
📡 API Endpoints
Method	Endpoint	Description	Body (JSON)
GET	/users	Fetch all users	N/A
POST	/users	Create a new user	{ "name": "John", "email": "john@example.com" }
PUT	/users/{id}	Update a user	{ "name": "Jane" }
DELETE	/users/{id}	Delete a user	N/A

🔒 Security Notes
Use .env for credentials instead of hardcoding them.

Use service role key for server-side actions.

Avoid exposing your Supabase key in frontend code.

🤝 Contributing
Pull requests are welcome!
For major changes, please open an issue first to discuss what you’d like to change.

📜 License
This project is licensed under the MIT License.
