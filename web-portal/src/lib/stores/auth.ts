// Authentication store using Svelte stores
import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import type { User } from '$lib/api';

interface AuthState {
	user: User | null;
	token: string | null;
	loading: boolean;
}

function createAuthStore() {
	const { subscribe, set, update } = writable<AuthState>({
		user: null,
		token: browser ? localStorage.getItem('auth_token') : null,
		loading: false
	});

	return {
		subscribe,
		setUser: (user: User, token: string) => {
			if (browser) {
				localStorage.setItem('auth_token', token);
			}
			set({ user, token, loading: false });
		},
		clearUser: () => {
			if (browser) {
				localStorage.removeItem('auth_token');
			}
			set({ user: null, token: null, loading: false });
		},
		setLoading: (loading: boolean) => {
			update(state => ({ ...state, loading }));
		}
	};
}

export const auth = createAuthStore();
