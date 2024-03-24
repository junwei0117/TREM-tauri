<script setup lang="ts">
defineEmits<{
  click: [payload: MouseEvent];
}>();
</script>

<template lang="pug">
button.chip(@click="$emit('click', $event)")
  span.leading-icon.material-symbols-rounded: slot(name="icon")
  span.label: slot(name="label")
</template>

<style lang="scss" scoped>
.chip {
  --chip-label: var(--text-color);
  --chip-label-hover: 0deg 0% 94%;
  --chip-label-active: 0deg 0% 100%;
  --chip-background: var(--surface-card);
  --chip-background-hover: color-mix(
    in lab,
    var(--surface-card),
    var(--highlight-bg)
  );
  --chip-background-active: var(--surface-card);
  --chip-outline: var(--surface-border);
  --chip-outline-hover: 0deg 0% 75%;

  all: unset;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px;
  padding-right: 10px;
  border-radius: 48px;
  cursor: pointer;
  background-color: var(--chip-background);
  color: var(--chip-label);
  transition-property: background-color, color, outline, opacity;
  transition-duration: 0.1s;
  transition-timing-function: cubic-bezier(0.2, 0, 0, 1);
  outline: 1px solid var(--chip-outline);

  &:disabled {
    cursor: not-allowed;
    opacity: 0.4;
  }

  > .leading-icon {
    height: 16px;
    aspect-ratio: 1;
    font-size: 16px;
    pointer-events: none;
  }

  > .label {
    line-height: 12px;
    font-size: 12px;
    pointer-events: none;
  }

  &:not(:disabled) {
    &:hover {
      background-color: var(--chip-background-hover);
      color: var(--chip-label-hover);
      outline-color: var(--chip-outline-hover);
    }

    &:active {
      background-color: var(--chip-background-active);
      color: var(--chip-label-active);
    }
  }
}
</style>
