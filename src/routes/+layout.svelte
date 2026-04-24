<script lang="ts">
  import "./layout.css";

  const { children } = $props();
</script>

<div class="p-2 w-full h-full max-h-146 flex flex-col max-w-xl border-2">
  {@render children()}
</div>
