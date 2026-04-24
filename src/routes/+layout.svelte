<script lang="ts">
  import "./layout.css";

  const { children } = $props();
</script>

<div class="p-2 w-full flex flex-col max-w-xl">
  {@render children()}
</div>
