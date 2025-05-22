<script>
  import { onMount } from 'svelte';
  let packages = [];
  let installing = '';
  let output = '';
  let search = '';
  let source = 'Arch';
  let githubApps = [];
  let showGithub = false;

  async function fetchPackages() {
    if (source === 'Arch' || source === 'ChaoticAUR') {
      // @ts-ignore
      packages = await window.__TAURI__.invoke('list_packages');
      if (source === 'ChaoticAUR') {
        packages = packages.filter(pkg => pkg.repo === 'ChaoticAUR');
      } else {
        packages = packages.filter(pkg => pkg.repo !== 'AUR');
      }
    } else if (source === 'AUR') {
      // @ts-ignore
      packages = await window.__TAURI__.invoke('search_aur', { query: search || 'yay' });
    }
  }

  async function packageAction(action, pkg) {
    installing = pkg.name;
    // @ts-ignore
    output = await window.__TAURI__.invoke('package_action', { action, pkg: pkg.name });
    installing = '';
  }

  function setSource(s) {
    source = s;
    showGithub = false;
    fetchPackages();
  }

  function doSearch() {
    fetchPackages();
  }

  async function fetchGithubApps() {
    // @ts-ignore
    githubApps = await window.__TAURI__.invoke('list_github_apps');
    showGithub = true;
  }

  onMount(fetchPackages);
</script>

<main>
  <h1>GhostView (Octopi Clone)</h1>
  <div style="margin-bottom:1em;">
    <button on:click={() => setSource('Arch')}>Arch</button>
    <button on:click={() => setSource('AUR')}>AUR</button>
    <button on:click={() => setSource('ChaoticAUR')}>ChaoticAUR</button>
    <button on:click={fetchGithubApps}>GitHub Apps</button>
    <input placeholder="Search" bind:value={search} on:keydown={(e) => e.key === 'Enter' && doSearch()} />
    <button on:click={doSearch}>Search</button>
  </div>
  {#if showGithub}
    <h2>Curated GitHub Apps</h2>
    <ul>
      {#each githubApps as app}
        <li>
          <b>{app.name}</b> <a href={app.url} target="_blank">{app.repo}</a> — {app.desc}
          <!-- Future: Add install button -->
        </li>
      {/each}
    </ul>
  {:else}
    <ul>
      {#each packages as pkg}
        <li>
          <b>{pkg.name}</b> <small>[{pkg.repo}]</small> — {pkg.desc}
          <button on:click={() => packageAction('install', pkg)} disabled={installing === pkg.name}>
            {installing === pkg.name ? 'Installing...' : 'Install'}
          </button>
          <button on:click={() => packageAction('remove', pkg)} disabled={installing === pkg.name}>
            Remove
          </button>
          <button on:click={() => packageAction('update', pkg)} disabled={installing === pkg.name}>
            Update
          </button>
        </li>
      {/each}
    </ul>
  {/if}
  {#if output}
    <pre>{output}</pre>
  {/if}
</main>

<style>
main {
  max-width: 700px;
  margin: 2rem auto;
  font-family: sans-serif;
}
button {
  margin-left: 0.5rem;
}
pre {
  background: #222;
  color: #eee;
  padding: 1em;
  border-radius: 6px;
  margin-top: 2em;
}
</style>