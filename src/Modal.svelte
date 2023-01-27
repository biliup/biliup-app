<script context="module">
    let totalComponents = 0;
</script>

<script>
    import {onMount} from "svelte";
    let topDiv;
    let componentId = totalComponents++;
    onMount(() => {
        document.body.appendChild(topDiv);
    });
</script>

<!-- The button to open modal -->
<!-- 占位显示标签 -->
<label for="component-modal-{componentId}" class="modal-button">
    <!-- NOTE for `aria-hidden`: 此处的标签实际上键盘不可操作，故忽略 -->
    <slot name="open-modal" aria-hidden="true">
    </slot>
</label>

<!-- Put this part before </body> tag -->
<div bind:this={topDiv}>
    <input type="checkbox" id="component-modal-{componentId}" class="modal-toggle">
    <label for="component-modal-{componentId}" class="modal cursor-pointer">
        <label class="modal-box">
            <slot name="box" componentId="component-modal-{componentId}">
                <h3 class="text-lg font-bold">Congratulations random Interner user!</h3>
                <p class="py-4">You've been selected for a chance to get one year of subscription to use Wikipedia for free!</p>
            </slot>
        </label>
    </label>
</div>

