<script lang="ts">
	import { goto } from '$app/navigation';
	import { api } from '$lib/api';
	import { auth } from '$lib/stores/auth';
	import { loginSchema } from '$lib/validation';

	let username = '';
	let password = '';
	let error = '';
	let loading = false;
	let errors: Record<string, string> = {};

	async function handleSubmit() {
		error = '';
		errors = {};
		loading = true;

		// Validate
		const result = loginSchema.safeParse({ username, password });
		if (!result.success) {
			result.error.errors.forEach((err) => {
				errors[err.path[0]] = err.message;
			});
			loading = false;
			return;
		}

		// Call API
		const response = await api.login({ username, password });

		if (response.error) {
			error = response.error;
			loading = false;
			return;
		}

		if (response.data) {
			auth.setUser(response.data.user, response.data.token);
			goto('/dashboard');
		}

		loading = false;
	}
</script>

<svelte:head>
	<title>Login - Assignment W-4</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center px-4 py-12 sm:px-6 lg:px-8">
	<div class="w-full max-w-md space-y-8">
		<div>
			<h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">
				Sign in to your account
			</h2>
			<p class="mt-2 text-center text-sm text-gray-600">
				Or
				<a href="/register" class="font-medium text-indigo-600 hover:text-indigo-500">
					create a new account
				</a>
			</p>
		</div>

		<form class="mt-8 space-y-6" on:submit|preventDefault={handleSubmit}>
			{#if error}
				<div class="rounded-md bg-red-50 p-4">
					<p class="text-sm text-red-800">{error}</p>
				</div>
			{/if}

			<div class="-space-y-px rounded-md shadow-sm">
				<div>
					<label for="username" class="sr-only">Username</label>
					<input
						id="username"
						name="username"
						type="text"
						required
						bind:value={username}
						class="relative block w-full rounded-t-md border-0 px-3 py-2 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
						placeholder="Username"
					/>
					{#if errors.username}
						<p class="mt-1 text-sm text-red-600">{errors.username}</p>
					{/if}
				</div>
				<div>
					<label for="password" class="sr-only">Password</label>
					<input
						id="password"
						name="password"
						type="password"
						required
						bind:value={password}
						class="relative block w-full rounded-b-md border-0 px-3 py-2 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:z-10 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
						placeholder="Password"
					/>
					{#if errors.password}
						<p class="mt-1 text-sm text-red-600">{errors.password}</p>
					{/if}
				</div>
			</div>

			<div class="flex items-center justify-between">
				<div class="flex items-center">
					<input
						id="remember-me"
						name="remember-me"
						type="checkbox"
						class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
					/>
					<label for="remember-me" class="ml-2 block text-sm text-gray-900">
						Remember me
					</label>
				</div>

				<div class="text-sm">
					<a href="/forgot-password" class="font-medium text-indigo-600 hover:text-indigo-500">
						Forgot your password?
					</a>
				</div>
			</div>

			<div>
				<button
					type="submit"
					disabled={loading}
					class="group relative flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:opacity-50"
				>
					{loading ? 'Signing in...' : 'Sign in'}
				</button>
			</div>
		</form>
	</div>
</div>
