// API client for communicating with Rust backend services
import { browser } from '$app/environment';

const API_BASE_URL = browser
	? (import.meta.env.VITE_API_URL || 'http://localhost:8084')
	: (process.env.API_URL || 'http://auth-service:8080');

interface ApiResponse<T> {
	data?: T;
	error?: string;
}

export interface User {
	id: string;
	username: string;
	email: string;
	created_at: string;
}

export interface LoginRequest {
	username: string;
	password: string;
}

export interface RegisterRequest {
	username: string;
	email: string;
	password: string;
}

export interface AuthResponse {
	user: User;
	token: string;
}

class ApiClient {
	private baseUrl: string;

	constructor(baseUrl: string) {
		this.baseUrl = baseUrl;
	}

	private async request<T>(
		endpoint: string,
		options: RequestInit = {}
	): Promise<ApiResponse<T>> {
		try {
			const token = browser ? localStorage.getItem('auth_token') : null;

			const headers: HeadersInit = {
				'Content-Type': 'application/json',
				...options.headers,
			};

			if (token) {
				headers['Authorization'] = `Bearer ${token}`;
			}

			const response = await fetch(`${this.baseUrl}${endpoint}`, {
				...options,
				headers,
			});

			if (!response.ok) {
				const error = await response.json().catch(() => ({ error: response.statusText }));
				return { error: error.error || error.message || 'Request failed' };
			}

			const data = await response.json();
			return { data };
		} catch (error) {
			return { error: error instanceof Error ? error.message : 'Network error' };
		}
	}

	async login(credentials: LoginRequest): Promise<ApiResponse<AuthResponse>> {
		return this.request<AuthResponse>('/api/auth/login', {
			method: 'POST',
			body: JSON.stringify(credentials),
		});
	}

	async register(data: RegisterRequest): Promise<ApiResponse<AuthResponse>> {
		return this.request<AuthResponse>('/api/auth/register', {
			method: 'POST',
			body: JSON.stringify(data),
		});
	}

	async logout(): Promise<ApiResponse<void>> {
		if (browser) {
			localStorage.removeItem('auth_token');
		}
		return { data: undefined };
	}

	async getCurrentUser(): Promise<ApiResponse<User>> {
		return this.request<User>('/api/auth/me');
	}

	async refreshToken(): Promise<ApiResponse<{ token: string }>> {
		return this.request<{ token: string }>('/api/auth/refresh', {
			method: 'POST',
		});
	}
}

export const api = new ApiClient(API_BASE_URL);
