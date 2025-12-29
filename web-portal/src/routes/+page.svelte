<svelte:head>
	<title>Assignment W-4 - Programmable World Engine</title>
</svelte:head>

<div class="relative overflow-hidden bg-white">
	<div class="mx-auto max-w-7xl">
		<div class="relative z-10 bg-white pb-8 sm:pb-16 md:pb-20 lg:w-full lg:max-w-2xl lg:pb-28 xl:pb-32">
			<main class="mx-auto mt-10 max-w-7xl px-4 sm:mt-12 sm:px-6 md:mt-16 lg:mt-20 lg:px-8 xl:mt-28">
				<div class="sm:text-center lg:text-left">
					<h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl md:text-6xl">
						<span class="block">Build Your Own</span>
						<span class="block text-indigo-600">Programmable World</span>
					</h1>
					<p class="mt-3 text-base text-gray-500 sm:mx-auto sm:mt-5 sm:max-w-xl sm:text-lg md:mt-5 md:text-xl lg:mx-0">
						A hybrid MUD/MUSH + graphical game engine. Create persistent worlds with custom rules,
						art, and gameplay using an embedded scripting language.
					</p>
					<div class="mt-5 sm:mt-8 sm:flex sm:justify-center lg:justify-start">
						<div class="rounded-md shadow">
							<a
								href="/register"
								class="flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 md:px-10 md:py-4 md:text-lg"
							>
								Get Started
							</a>
						</div>
						<div class="mt-3 sm:ml-3 sm:mt-0">
							<a
								href="/worlds"
								class="flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-100 px-8 py-3 text-base font-medium text-indigo-700 hover:bg-indigo-200 md:px-10 md:py-4 md:text-lg"
							>
								Explore Worlds
							</a>
						</div>
					</div>
				</div>
			</main>
		</div>
	</div>
</div>

<div class="bg-gray-50 py-12">
	<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
		<div class="lg:text-center">
			<h2 class="text-base font-semibold uppercase tracking-wide text-indigo-600">Features</h2>
			<p class="mt-2 text-3xl font-bold leading-8 tracking-tight text-gray-900 sm:text-4xl">
				Everything you need to build worlds
			</p>
		</div>

		<div class="mt-10">
			<dl class="space-y-10 md:grid md:grid-cols-2 md:gap-x-8 md:gap-y-10 md:space-y-0">
				<div class="relative">
					<dt>
						<div class="absolute flex h-12 w-12 items-center justify-center rounded-md bg-indigo-500 text-white">
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
							</svg>
						</div>
						<p class="ml-16 text-lg font-medium leading-6 text-gray-900">
							In-World Scripting
						</p>
					</dt>
					<dd class="ml-16 mt-2 text-base text-gray-500">
						Program game logic directly in-world using a safe, sandboxed scripting language.
					</dd>
				</div>

				<div class="relative">
					<dt>
						<div class="absolute flex h-12 w-12 items-center justify-center rounded-md bg-indigo-500 text-white">
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
							</svg>
						</div>
						<p class="ml-16 text-lg font-medium leading-6 text-gray-900">
							Multi-Protocol
						</p>
					</dt>
					<dd class="ml-16 mt-2 text-base text-gray-500">
						Connect via graphical clients (UDP) or text-based interfaces (Telnet/Web).
					</dd>
				</div>

				<div class="relative">
					<dt>
						<div class="absolute flex h-12 w-12 items-center justify-center rounded-md bg-indigo-500 text-white">
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
							</svg>
						</div>
						<p class="ml-16 text-lg font-medium leading-6 text-gray-900">
							Persistent Worlds
						</p>
					</dt>
					<dd class="ml-16 mt-2 text-base text-gray-500">
						All changes persist to the database. Your world evolves over time.
					</dd>
				</div>

				<div class="relative">
					<dt>
						<div class="absolute flex h-12 w-12 items-center justify-center rounded-md bg-indigo-500 text-white">
							<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
							</svg>
						</div>
						<p class="ml-16 text-lg font-medium leading-6 text-gray-900">
							Microservices Architecture
						</p>
					</dt>
					<dd class="ml-16 mt-2 text-base text-gray-500">
						Independent scaling and deployment. Update one service without restarting everything.
					</dd>
				</div>
			</dl>
		</div>
	</div>
</div>
