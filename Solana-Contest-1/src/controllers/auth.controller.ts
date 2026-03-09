import { Request, Response } from "express";
import * as authService from "../services/auth.service";
import { asyncHandler, AppError } from "../utils/errors";
import { requireFields, isValidEmail } from "../utils/validators";

export const register = asyncHandler(async (req: Request, res: Response) => {
    if (!req.body) throw new AppError("Missing request body", 400);
    requireFields(req.body, ["email", "password", "name"]);

    const { email, password, name } = req.body;

    if (!isValidEmail(email)) {
        throw new AppError("Invalid email format", 400);
    }

    const result = await authService.register({ email, password, name });
    res.status(201).json(result);
});

export const login = asyncHandler(async (req: Request, res: Response) => {
    requireFields(req.body, ["email", "password"]);

    const { email, password } = req.body;

    const result = await authService.login({ email, password });
    res.status(200).json(result);
});
