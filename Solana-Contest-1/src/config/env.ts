import dotenv from "dotenv";
dotenv.config();

export const DATABASE_URL = process.env.DATABASE_URL as string;
export const JWT_SECRET = process.env.JWT_SECRET as string;
export const PORT = parseInt(process.env.PORT || "3000", 10);
