<script>
  import { onMount } from 'svelte';
  let packages = [];
  let installing = '';
  let output = '';
  let search = '';
  let source = 'Arch';

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

  async function install(pkg) {
    installing = pkg.name;
    // @ts-ignore
    output = await window.__TAURI__.invoke('install_package', { pkg: pkg.name });
    installing = '';
  }

  function setSource(s) {
    source = s;
    fetchPackages();
  }

  function doSearch() {
    fetchPackages();
  }

  onMount(fetchPackages);
</script>

<main>
  <h1>GhostView (Octopi Clone)</h1>
  <div style="margin-bottom:1em;">
    <button on:click={() => setSource('Arch')}>Arch</button>
    <button on:click={() => setSource('AUR')}>AUR</button>
    <button on:click={() => setSource('ChaoticAUR')}>ChaoticAUR</button>
    <input placeholder="Search" bind:value={search} on:keydown={(e) => e.key === 'Enter' && doSearch()} />
    <button on:click={doSearch}>Search</button>
  </div>
  <ul>
    {#each packages as pkg}
      <li>
        <b>{pkg.name}</b> <small>[{pkg.repo}]</small> — {pkg.desc}
        <button on:click={() => install(pkg)} disabled={installing === pkg.name}>
          {installing === pkg.name ? 'Installing...' : 'Install'}
        </button>
      </li>
    {/each}
  </ul>
  {#if output}
    <pre>{output}</pre>
  {/if}
</main>

<style>
main {
  max-width: 600px;
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