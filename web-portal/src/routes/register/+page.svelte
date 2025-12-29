<script lang="ts">
	import { goto } from '$app/navigation';
	import { api } from '$lib/api';
	import { auth } from '$lib/stores/auth';
	import { registerSchema } from '$lib/validation';

	let username = '';
	let email = '';
	let password = '';
	let confirmPassword = '';
	let error = '';
	let loading = false;
	let errors: Record<string, string> = {};

	async function handleSubmit() {
		error = '';
		errors = {};
		loading = true;

		// Validate
		const result = registerSchema.safeParse({ username, email, password, confirmPassword });
		if (!result.success) {
			result.error.errors.forEach((err) => {
				errors[err.path[0]] = err.message;
			});
			loading = false;
			return;
		}

		// Call API
		const response = await api.register({ username, email, password });

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
	<title>Register - Assignment W-4</title>
</svelte:head>

<div class="flex min-h-full items-center justify-center px-4 py-12 sm:px-6 lg:px-8">
	<div class="w-full max-w-md space-y-8">
		<div>
			<h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">
				Create your account
			</h2>
			<p class="mt-2 text-center text-sm text-gray-600">
				Already have an account?
				<a href="/login" class="font-medium text-indigo-600 hover:text-indigo-500">
					Sign in
				</a>
			</p>
		</div>

		<form class="mt-8 space-y-6" on:submit|preventDefault={handleSubmit}>
			{#if error}
				<div class="rounded-md bg-red-50 p-4">
					<p class="text-sm text-red-800">{error}</p>
				</div>
			{/if}

			<div class="space-y-4">
				<div>
					<label for="username" class="block text-sm font-medium text-gray-700">Username</label>
					<input
						id="username"
						name="username"
						type="text"
						required
						bind:value={username}
						class="mt-1 block w-full rounded-md border-0 px-3 py-2 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
						placeholder="username"
					/>
					{#if errors.username}
						<p class="mt-1 text-sm text-red-600">{errors.username}</p>
					{/if}
				</div>

				<div>
					<label for="email" class="block text-sm font-medium text-gray-700">Email</label>
					<input
						id="email"
						name="email"
						type="email"
						required
						bind:value={email}
						class="mt-1 block w-full rounded-md border-0 px-3 py-2 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
						placeholder="you@example.com"
					/>
					{#if errors.email}
						<p class="mt-1 text-sm text-red-600">{errors.email}</p>
					{/if}
				</div>

				<div>
					<label for="password" class="block text-sm font-medium text-gray-700">Password</label>
					<input
						id="password"
						name="password"
						type="password"
						required
						bind:value={password}
						class="mt-1 block w-full rounded-md border-0 px-3 py-2 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
						placeholder="••••••••"
					/>
					{#if errors.password}
						<p class="mt-1 text-sm text-red-600">{errors.password}</p>
					{/if}
					<p class="mt-1 text-xs text-gray-500">
						Must be at least 8 characters with uppercase, lowercase, and numbers
					</p>
				</div>

				<div>
					<label for="confirm-password" class="block text-sm font-medium text-gray-700">
						Confirm Password
					</label>
					<input
						id="confirm-password"
						name="confirm-password"
						type="password"
						required
						bind:value={confirmPassword}
						class="mt-1 block w-full rounded-md border-0 px-3 py-2 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
						placeholder="••••••••"
					/>
					{#if errors.confirmPassword}
						<p class="mt-1 text-sm text-red-600">{errors.confirmPassword}</p>
					{/if}
				</div>
			</div>

			<div>
				<button
					type="submit"
					disabled={loading}
					class="group relative flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:opacity-50"
				>
					{loading ? 'Creating account...' : 'Create account'}
				</button>
			</div>

			<p class="text-center text-xs text-gray-500">
				By registering, you agree to our
				<a href="/terms" class="text-indigo-600 hover:text-indigo-500">Terms of Service</a>
				and
				<a href="/privacy" class="text-indigo-600 hover:text-indigo-500">Privacy Policy</a>
			</p>
		</form>
	</div>
</div>
