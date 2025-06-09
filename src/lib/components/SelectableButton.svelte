<script lang="ts">
    // Required prop for the button's display text
    export let text: string;

    // Optional prop to determine if the button is in a 'selected' state for styling
    export let selected: boolean = false;

    // Capture all props passed to the component.
    // We'll destructure 'class' to handle it specially and spread the rest.
    const { class: additionalClasses, ...restProps } = $$props;

    // Define base and conditional styling classes
    const baseStyling = "px-3 py-2 border rounded text-center hover:bg-slate-700";
    const selectedStyling = 'bg-green-900 border-green-700 text-green-400';
    const unselectedStyling = 'bg-slate-900 border-slate-700 text-slate-300'; // Added text-slate-300 for consistency

    // Reactive statement to compute the final class string.
    // This ensures that if 'selected' or 'additionalClasses' (passed from parent) change,
    // the class list updates.
    $: finalClassList = `
    ${baseStyling}
    ${selected ? selectedStyling : unselectedStyling}
    ${additionalClasses || ''}
  `.replace(/\s+/g, ' ').trim(); // Normalize whitespace and trim for a clean class string
</script>

<button
        type="button"
        class={finalClassList}
        {...restProps}
>
{text}
</button>