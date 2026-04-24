<script lang="ts">
  import "./layout.css";

  const { children } = $props();
</script>

<div class="p-2 w-full min-h-146 max-w-xl border-2">
  {@render children()}
</div>
