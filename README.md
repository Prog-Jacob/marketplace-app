# Marketplace App Template

The Marketplace App Template is a comprehensive solution designed with scalability in mind, encompassing both the backend and frontend components of a marketplace application. Leveraging Rust for the backend and React.js for the frontend, the project integrates a range of cutting-edge technologies to deliver a robust and efficient marketplace experience.

> **_NOTE:_**
> Check out the [Getting Started Guide](GETTINGSTARTED.md) and the [SCHEMATICS](SCHEMATICS.md) for in-depth walkthrough!

## Key Technologies:

- **API: Tonic Rust** 
  - The backend, written in Rust, utilizes the gRPC (Google Remote Procedure Call) framework for efficient and high-performance communication between services.

- **Database: Sqlx Postgres**
  - The project employs a Postgres database for reliable and scalable data storage, supporting the dynamic requirements of a marketplace.

- **Frontend: React.js TypeScript**
  - The frontend is built using React.js, providing a responsive and user-friendly interface for seamless user interactions.

## Features:

- **Search Engine Integration (Meilisearch):**
  - Leveraging Meilisearch, the project incorporates a powerful search engine to enhance discoverability and navigation within the marketplace.

- **Recommender System:**
  - Future plans include the integration of a recommender system, with options to either develop a custom solution utilizing machine learning techniques or adopt proven solutions like Gorse.

- **Payment Processing with Stripe:**
  - The payment process is facilitated by Stripe, ensuring secure and seamless transactions within the marketplace.

## Next Steps:

Continued development will focus on refining existing features, implementing the recommender system, and exploring additional functionalities to enrich the user experience. Additionally, upon further experimentation, inclusion of a NoSQL database solution will most likely be made. Finally, setting up a robust CI/CD environment is crucial for production demands, so CircleCI, way to go!

## License:

I don't know much about this stuff. That being said, feel free to use this work at any scale.

## Acknowledgments:

Huge high-fives to the awesome peeps behind [Tonic](https://github.com/hyperium/tonic), [SQLx](https://github.com/launchbadge/sqlx), and [Meilisearch](https://github.com/meilisearch/meilisearch) for making the backend magic happen. Mad props to the [React.js](https://github.com/facebook/react) and [gRPC-web](https://github.com/grpc/grpc-web) crews for the frontend awesomeness. Big thanks to the [Envoy](https://github.com/envoyproxy/envoy) gang for being the edge and service proxy heroes. Special love to [OWASP](https://owasp.org/) and [Protocol Buffers](https://developers.google.com/protocol-buffers) for the killer resources. Shoutout to the cool cats at [Vertabelo](https://vertabelo.com/blog/user-authentication-module/), [Fabric Inc.](https://fabric.inc/blog/commerce/ecommerce-database-design-example), and this [Medium article](https://medium.com/@f.pitterling/stripe-integration-with-react-flask-e7aac635b139) for dropping knowledge bombs! 🚀✨