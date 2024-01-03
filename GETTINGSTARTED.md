# Getting Started

Follow these steps to quickly set up and run the Marketplace App Template on your local machine. This guide assumes you have Rust and Node.js installed. If not, please install them before proceeding.

> **_NOTE:_**
> Check out the [SCHEMATICS](SCHEMATICS.md) for in-depth design!

## Backend Setup

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/Prog-Jacob/marketplace-app.git
   cd marketplace-app/backend
   ```

2. **Install Dependencies:**
   ```bash
   cargo build
   ```

3. **Run the Database Migrations:**
   ```bash
   // To Be Implemented
   ```

4. **Run the Backend Server:**
   ```bash
   cargo run --bin server
   ```

   The backend server should be running at `http://localhost:$BACKEND_PORT$`.

## Frontend Setup

1. **Navigate to the Frontend Directory:**
   ```bash
   cd marketplace-app/frontend
   ```

2. **Install Dependencies:**
   ```bash
   npm install
   ```

3. **Run the Frontend Development Server:**
   ```bash
   npm start
   ```

   The frontend development server should be running at `http://localhost:$FRONTEND_PORT$`.

## Accessing the Application

Visit the frontend development URL in your web browser to access the Marketplace App Template. You can explore the frontend interface and interact with the application.

## Environment Variables

#### Backend:

1. **Database Connection:**
   - `DATABASE_URL`: The full connection URL for your PostgreSQL database, including username and password.
   - `TEST_DATABASE_URL`: Database connection URL specifically for development.
   - `DATABASE_POOL_SIZE`: The number of database connections in the pool.

2. **Authentication and Security:**
   - `JWT_SECRET`: Secret key for JSON Web Token (JWT) authentication.
   - `HASH_PEPPER`: Secret key for password hashing.
   - `SALT_ROUNDS`: Number of salting rounds in password hashing.

3. **Environment Configuration:**
   - `SERVER_ENV`: Specify whether the app is running in "DEV" or "PROD".
   - `SERVER_PORT`: Port on which the application should listen.
   - `PROTO_PATH`: Path to the protocol buffers definitions.
   
4. Third-Party Service Keys:
   - `STRIPE_PRIVATE_KEY`: Stripe's confidential server-side key for secure authentication and interaction with the Stripe API.
   - `AWS_BUCKET`: Name of the Amazon S3 bucket serving as a cloud storage repository for the application.
   - `AWS_REGION`: AWS region geographic location for the specified Amazon S3 bucket.
   - `AWS_PROFILE`: AWS CLI configuration profile name for authentication and authorization in AWS service interactions.
   
5. IAM Credentials:
   - `AWS_ACCESS_KEY_ID`
   - `AWS_SECRET_ACCESS_KEY`
   - `AWS_DEFAULT_REGION`

In order to deploy to AWS, the IAM credentials should be set in your CI/CD service provider if any, or else on your machine.

#### Frontend:

1. **API Endpoints:**
   - `FRONT_BASE_URL`: Base URL for the backend API.

2. **Third-Party Service Keys:**
   - `STRIPE_PUBLIC_KEY`: Stripe public key for client-side integration.

2. **Deployment and Configuration:**
   - `PROTO_PATH`: Path to the protocol buffers definitions.
   - `FRONT_PORT`: Specify the port your application should listen on.
   - `FRONT_HOST`: The host address for the application.


## File Structure

```
marketplace-app/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── routes/
│   │   │   │   └── mod.rs
│   │   │   └── controllers/
│   │   │       └── mod.rs
│   │   ├── models/
│   │   │   └── mod.rs
│   │   ├── services/
│   │   │   └── mod.rs
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   └── connection.rs
│   │   ├── utils/
│   │   │   └── mod.rs
│   ├── tests/
│   ├── Cargo.toml
│   ├── build.rs
├── shared/
│   └── proto/
│       └── example.proto
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── Header/
│   │   │   │   ├── Header.ts
│   │   │   │   ├── Header.css
│   │   │   │   └── ...
│   │   │   └── ...
│   │   ├── views/
│   │   │   ├── Home/
│   │   │   │   ├── Home.ts
│   │   │   │   ├── Home.css
│   │   │   │   └── ...
│   │   │   └── ...
│   │   ├── styles/
│   │   │   ├── global.css
│   │   │   └── ...
│   │   ├── App.ts
│   │   └── index.ts
│   ├── public/
│   ├── tests/
│   ├── package.json
│   └── ...
```

## Usage
> - **`backend/src/main.rs`**:
> This is the entry point of your application. It should set up the server, bind the gRPC services, and start the server.
    
> - **`backend/src/api/mod.rs`**:
> This module should import and expose all the API endpoints of your application.
    
> - **`backend/src/api/routes/mod.rs`**:
> This module should define all the routes for your API.
    
> - **`backend/src/api/controllers/mod.rs`**:
> This module should contain the logic for handling requests and forming responses.
    
> - **`backend/src/models/mod.rs`**:
> This module should define the data structures that your application uses.
    
> - **`backend/src/services/mod.rs`**:
> This module should contain the business logic of your application. It interacts with the models and the database.
    
> - **`backend/src/db/mod.rs`**:
> This module should contain the logic for interacting with the database.
    
> - **`backend/src/db/connection.rs`**:
> This file should set up the database connection.
    
> - **`backend/src/utils/mod.rs`**:
> This module should contain utility functions and constants that are used throughout your application.
    
> - **`backend/tests`/**:
> This directory should contain all the test cases for your application.
    
> - **`backend/Cargo.toml`**:
> This file should list all the dependencies of your application.
    
> - **`backend/build.rs`**:
> This file should contain the build script for your application.
    
> - **`shared/proto/`**:
> This folder should define the protocol buffers definitions across all application.

> - **`frontend/src/components/`**:
> This directory should contain reusable React components. Each component should have its own folder with a `.ts` file for the component logic, a `.css` file for styling, and any other related files.
    
> - **`frontend/src/views/`**:
> This directory should contain higher-level components that represent entire views or pages of your application. Each view can have its own folder with a `.ts` file for the view logic, a `.css` file for styling, and any other related files.
    
> - **`frontend/src/styles/`**:
> This directory should contain global styles that are applied across multiple components and views. The `global.css` file, for example, can contain styles that affect the entire application.
    
> - **`frontend/src/App.ts`**:
> This file represents the main component of your React application. It typically includes the routing logic and renders the different views based on the current route.
    
> - **`frontend/src/index.ts`**:
> This file is the entry point of your React application. It renders the `App` component and is responsible for initializing the React app.
    
> - **`frontend/public/`**:
> This directory contains static assets that are publicly accessible, such as images or favicon.ico.
    
> - **`frontend/tests/`**:
> This directory should contain test files for your React components and other relevant code.
    
> - **`frontend/package.json`**:
> This file contains metadata about the project and its dependencies. It also includes scripts for running tasks such as building or testing the application.