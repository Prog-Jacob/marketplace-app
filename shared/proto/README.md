# Notes

1. **Separation of Concerns:** Organize your services based on their functionality. If you have multiple distinct features, consider grouping related RPC methods into separate services or files.

2. **Modularity:** Break down complex services into smaller, manageable parts. This can make your codebase more maintainable and scalable.

3. **Reusability:** Design your services to be reusable across different parts of your application. Avoid duplicating code or functionality.

4. **Single Responsibility Principle (SRP):** Each service or RPC method should have a clear and single responsibility. This makes your code easier to understand and maintain.

5. **Consistent Naming:** Use clear and consistent naming conventions for messages, services, and RPC methods. This helps maintain readability.

6. **Versioning:** Plan for future changes by considering versioning in your service definitions. This allows you to introduce updates without breaking existing clients.

7. **Documentation:** Include comments and documentation in your Protocol Buffers files to explain the purpose of messages, services, and methods. This is essential for other developers who might work with your code.

8. **Error Handling:** Design robust error handling mechanisms in your services. Define error messages that provide meaningful information to clients.

9. **Security Considerations:** Depending on your application's security requirements, consider incorporating authentication and authorization mechanisms into your services.


> __**Note To Self**__:
> I can't find a **sound** way to authentication. All methods I came across are comprising options against each other. I guess the best case is to assume that attackers would always have $ \lt{ AccessTokenTTL } $ time to act upon. Till further study, I would keep using the current authentication model with fallbacks on refresh tokens when the request is sensitive e.g. payment methods.

## Authentication Model:
In very short terms:
1. Generate access and refresh tokens on login.
2. Set them as cookies with flags HttpOnly, Secure, Expires, SameSite=Strict.
3. On each request (if necessary), validate the access token.
    - If it's expired, validate the refresh token.
        - If it's expired, demand the user to login.
        - If it's valid. Check if it's revoked.
            - If yes, demand the user to login.
            - Else, goto (1).
    - If it's valid, check if request is sensitive.
        - If sensitive, validate the refresh token.
        - Else, proceed.