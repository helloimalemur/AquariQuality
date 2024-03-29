/** @type {import('next').NextConfig} */
const nextConfig = {
    async headers() {
        return [
            {
                source: "/:path*",
                // headers: [
                //     { key: "Access-Control-Allow-Origin", value: "*" },
                //     { key: "Access-Control-Allow-Headers", value: "X-API-KEY" },
                //     { key: "Access-Control-Allow-Methods", value: ["GET", "POST", "OPTIONS"] },
                //
                // ]

                headers: [
                    // Allow for specific domains to have access or * for all
                    {
                        key: "Access-Control-Allow-Origin",
                        value: "*",
                        // DOES NOT WORK
                        // value: process.env.ALLOWED_ORIGIN,
                    },
                    // Allows for specific methods accepted
                    {
                        key: "Access-Control-Allow-Methods",
                        value: "GET, POST, PUT, DELETE, OPTIONS",
                    },
                    // Allows for specific headers accepted (These are a few standard ones)
                    {
                        key: "Access-Control-Allow-Headers",
                        value: "Content-Type, Authorization",
                    },
                ],
            }
        ]
    }
};

export default nextConfig;
