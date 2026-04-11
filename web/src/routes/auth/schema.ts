import { z } from 'zod';

export const loginSchema = z.object({
	username: z.string().min(1, 'Name is required'),
	password: z.string().min(1, 'Password is required')
});

export const registerSchema = z
	.object({
		username: z.string().min(1, 'Name is required'),
		password: z.string().min(8, 'Password must be at least 8 characters'),
		confirm: z.string().min(1, 'Confirm your password')
	})
	.refine((data) => data.password === data.confirm, {
		message: "Passwords don't match",
		path: ['confirm']
	});

export type LoginBody = z.infer<typeof loginSchema>;
export type RegisterBody = z.infer<typeof registerSchema>;
