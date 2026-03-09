import { AppError } from "./errors";

export const requireFields = (body: any, fields: string[]) => {
    const missing = fields.filter(
        (f) => body[f] === undefined || body[f] === null || body[f] === ""
    );
    if (missing.length > 0) {
        throw new AppError(`Missing required fields: ${missing.join(", ")}`, 400);
    }
};

export const isValidEmail = (email: string) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
