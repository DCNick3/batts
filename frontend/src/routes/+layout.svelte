<script>
  import "../style.css"
</script>

<div class="page">
  <div class="panel">
    <h1><a href="/">Batts</a></h1>
  </div>

  <slot />
</div>

<style>
  .page {
    display: flex;
    flex: 1 1 100%;
    height: 100vh;
  }

  .panel {
    display: flex;
    flex-direction: column;
    padding: 1em;
    background-color: rgba(214, 238, 248, 0.952);
    min-width: 200px;
  }

  h1 {
    margin: 0 auto;
  }

  a {
    text-decoration: none;
    color: black;
  }
</style>