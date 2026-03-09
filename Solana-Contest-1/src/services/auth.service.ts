import bcrypt from "bcryptjs";
import jwt from "jsonwebtoken";
import prisma from "../utils/prisma";
import { JWT_SECRET } from "../config/env";
import { AppError } from "../utils/errors";

const SALT_ROUNDS = 10;
const TOKEN_EXPIRY = "24h";

export const register = async ({ email, password, name }: any) => {
    const existing = await prisma.user.findUnique({ where: { email } });
    if (existing) {
        throw new AppError("Email already registered", 409);
    }

    const hashedPassword = await bcrypt.hash(password, SALT_ROUNDS);
    const user = await prisma.user.create({
        data: { email, password: hashedPassword, name },
    });

    const token = generateToken(user);
    return { token, user: sanitizeUser(user) };
};

export const login = async ({ email, password }: any) => {
    const user = await prisma.user.findUnique({ where: { email } });
    if (!user) {
        throw new AppError("Invalid credentials", 401);
    }

    const isMatch = await bcrypt.compare(password, user.password);
    if (!isMatch) {
        throw new AppError("Invalid credentials", 401);
    }

    const token = generateToken(user);
    return { token, user: sanitizeUser(user) };
};

const generateToken = (user: any) => {
    return jwt.sign({ id: user.id, email: user.email }, JWT_SECRET, {
        expiresIn: TOKEN_EXPIRY,
    });
};

const sanitizeUser = (user: any) => ({
    id: user.id,
    email: user.email,
    name: user.name,
});
