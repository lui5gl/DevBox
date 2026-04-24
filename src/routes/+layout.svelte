<script lang="ts">
  import "./layout.css";

  const { children } = $props();
</script>

<div class="p-2">
  {@render children()}
</div>
