import express from "express";
import cors from "cors";
import dotenv from "dotenv";
import { PORT } from "./config/env";
import { AppError } from "./utils/errors";

import healthRoutes from "./routes/health";
import authRoutes from "./routes/auth";
import launchRoutes from "./routes/launches";

dotenv.config();

const app = express();

app.use(cors());
app.use(express.json());

// TODO: GET /api/health
// TODO: POST /api/auth/register
// TODO: POST /api/auth/login
// TODO: POST /api/launches (with tiers?, vesting?)
// TODO: GET /api/launches (?page, ?limit, ?status)
// TODO: GET /api/launches/:id (with computed status)
// TODO: PUT /api/launches/:id
// TODO: POST /api/launches/:id/whitelist
// TODO: GET /api/launches/:id/whitelist
// TODO: DELETE /api/launches/:id/whitelist/:address
// TODO: POST /api/launches/:id/referrals
// TODO: GET /api/launches/:id/referrals
// TODO: POST /api/launches/:id/purchase (with referralCode?, tier pricing, sybil protection)
// TODO: GET /api/launches/:id/purchases
// TODO: GET /api/launches/:id/vesting?walletAddress=

app.use("/api/health", healthRoutes);
app.use("/api/auth", authRoutes);
app.use("/api/launches", launchRoutes);

app.use((_req, _res, next) => {
    next(new AppError("Route not found", 404));
});

app.use((err: any, _req: express.Request, res: express.Response, _next: express.NextFunction) => {
    const statusCode = err.statusCode || 500;
    const message = err.isOperational ? err.message : "Internal server error";

    // Log unexpected errors
    if (!err.isOperational) {
        console.error("Unexpected error:", err);
    }

    res.status(statusCode).json({ error: message });
});

app.listen(PORT, () => {
    console.log(`Solana Launchpad API running on port ${PORT}`);
});

export default app;
