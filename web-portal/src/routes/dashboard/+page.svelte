<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { auth } from '$lib/stores/auth';
	import { api } from '$lib/api';

	let user: any = null;
	let loading = true;

	onMount(async () => {
		const unsubscribe = auth.subscribe(state => {
			user = state.user;
			loading = state.loading;
		});

		// Fetch current user if we have a token but no user
		const currentState = await new Promise(resolve => {
			const unsub = auth.subscribe(resolve);
			unsub();
		});

		if (currentState.token && !currentState.user) {
			const response = await api.getCurrentUser();
			if (response.data) {
				auth.setUser(response.data, currentState.token);
			} else {
				// Token invalid, redirect to login
				auth.clearUser();
				goto('/login');
			}
		} else if (!currentState.token) {
			// Not logged in
			goto('/login');
		}

		return unsubscribe;
	});

	async function handleLogout() {
		await api.logout();
		auth.clearUser();
		goto('/');
	}
</script>

<svelte:head>
	<title>Dashboard - Assignment W-4</title>
</svelte:head>

{#if loading}
	<div class="flex min-h-screen items-center justify-center">
		<div class="text-center">
			<div class="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-indigo-600 border-r-transparent"></div>
			<p class="mt-2 text-gray-600">Loading...</p>
		</div>
	</div>
{:else if user}
	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<div class="mb-8 flex items-center justify-between">
			<h1 class="text-3xl font-bold text-gray-900">Dashboard</h1>
			<button
				on:click={handleLogout}
				class="rounded-md bg-gray-600 px-4 py-2 text-sm font-medium text-white hover:bg-gray-700"
			>
				Logout
			</button>
		</div>

		<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			<!-- User Info Card -->
			<div class="overflow-hidden rounded-lg bg-white shadow">
				<div class="px-4 py-5 sm:p-6">
					<h3 class="text-lg font-medium leading-6 text-gray-900">Account Information</h3>
					<dl class="mt-4 space-y-2">
						<div>
							<dt class="text-sm font-medium text-gray-500">Username</dt>
							<dd class="text-sm text-gray-900">{user.username}</dd>
						</div>
						<div>
							<dt class="text-sm font-medium text-gray-500">Email</dt>
							<dd class="text-sm text-gray-900">{user.email}</dd>
						</div>
						<div>
							<dt class="text-sm font-medium text-gray-500">Member Since</dt>
							<dd class="text-sm text-gray-900">
								{new Date(user.created_at).toLocaleDateString()}
							</dd>
						</div>
					</dl>
				</div>
			</div>

			<!-- Characters Card -->
			<div class="overflow-hidden rounded-lg bg-white shadow">
				<div class="px-4 py-5 sm:p-6">
					<h3 class="text-lg font-medium leading-6 text-gray-900">Your Characters</h3>
					<p class="mt-4 text-sm text-gray-500">
						You don't have any characters yet.
					</p>
					<button class="mt-4 rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700">
						Create Character
					</button>
				</div>
			</div>

			<!-- Worlds Card -->
			<div class="overflow-hidden rounded-lg bg-white shadow">
				<div class="px-4 py-5 sm:p-6">
					<h3 class="text-lg font-medium leading-6 text-gray-900">Available Worlds</h3>
					<p class="mt-4 text-sm text-gray-500">
						No worlds available yet.
					</p>
					<a
						href="/worlds"
						class="mt-4 inline-block rounded-md bg-indigo-100 px-4 py-2 text-sm font-medium text-indigo-700 hover:bg-indigo-200"
					>
						Browse Worlds
					</a>
				</div>
			</div>
		</div>

		<!-- Quick Stats -->
		<div class="mt-8">
			<h2 class="text-xl font-semibold text-gray-900">Quick Stats</h2>
			<div class="mt-4 grid grid-cols-1 gap-5 sm:grid-cols-3">
				<div class="overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:p-6">
					<dt class="truncate text-sm font-medium text-gray-500">Active Characters</dt>
					<dd class="mt-1 text-3xl font-semibold tracking-tight text-gray-900">0</dd>
				</div>
				<div class="overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:p-6">
					<dt class="truncate text-sm font-medium text-gray-500">Worlds Joined</dt>
					<dd class="mt-1 text-3xl font-semibold tracking-tight text-gray-900">0</dd>
				</div>
				<div class="overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:p-6">
					<dt class="truncate text-sm font-medium text-gray-500">Hours Played</dt>
					<dd class="mt-1 text-3xl font-semibold tracking-tight text-gray-900">0</dd>
				</div>
			</div>
		</div>
	</div>
{/if}
