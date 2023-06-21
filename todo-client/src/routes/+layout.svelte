<script>
  // Your selected Skeleton theme:
  import "@skeletonlabs/skeleton/themes/theme-crimson.css";

  // This contains the bulk of Skeletons required styles:
  import "@skeletonlabs/skeleton/styles/skeleton.css";

  // Finally, your application's global stylesheet (sometimes labeled 'app.css')
  import "../app.postcss";
  import { LightSwitch } from "@skeletonlabs/skeleton";
</script>

<div class="absolute mt-4 right-4">
  <LightSwitch />
</div>

<slot />
