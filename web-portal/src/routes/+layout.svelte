<script lang="ts">
	import '../app.css';
</script>

<div class="min-h-screen bg-gray-50">
	<nav class="bg-white shadow-sm">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="flex h-16 justify-between">
				<div class="flex">
					<div class="flex flex-shrink-0 items-center">
						<a href="/" class="text-xl font-bold text-gray-900">Assignment W-4</a>
					</div>
					<div class="hidden sm:ml-6 sm:flex sm:space-x-8">
						<a
							href="/"
							class="inline-flex items-center border-b-2 border-transparent px-1 pt-1 text-sm font-medium text-gray-900 hover:border-gray-300"
						>
							Home
						</a>
						<a
							href="/worlds"
							class="inline-flex items-center border-b-2 border-transparent px-1 pt-1 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700"
						>
			Worlds
						</a>
					</div>
				</div>
				<div class="hidden sm:ml-6 sm:flex sm:items-center">
					<a
						href="/login"
						class="rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:text-gray-900"
					>
						Sign in
					</a>
					<a
						href="/register"
						class="ml-4 rounded-md bg-indigo-600 px-3 py-2 text-sm font-medium text-white hover:bg-indigo-700"
					>
						Register
					</a>
				</div>
			</div>
		</div>
	</nav>

	<main>
		<slot />
	</main>

	<footer class="bg-white">
		<div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
			<p class="text-center text-sm text-gray-500">
				&copy; 2025 Assignment W-4. Built with Rust + SvelteKit.
			</p>
		</div>
	</footer>
</div>
