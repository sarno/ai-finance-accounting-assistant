<template>
  <div ref="rootRef" :class="['searchable-dropdown', sizeClass, containerClass]">
    <div class="searchable-dropdown-wrapper">
      <button
        v-if="!isOpen"
        type="button"
        :class="['searchable-dropdown-display', inputSizeClass, inputClass]"
        :disabled="disabled"
        @click.stop="openDropdown"
      >
        <span class="searchable-dropdown-display-text" :class="{ 'is-placeholder': !selectedOption }">
          <slot name="selected" :option="selectedOption">
            {{ selectedOption ? getOptionLabel(selectedOption) : placeholder }}
          </slot>
        </span>
        <span class="searchable-dropdown-arrow">▼</span>
      </button>

      <input
        v-else
        v-model="searchTerm"
        type="text"
        :class="['form-input', inputSizeClass, inputClass]"
        :placeholder="placeholder"
        :disabled="disabled"
        @focus="openDropdown"
        @input="handleInput"
        @click.stop="openDropdown"
      />

      <div v-show="isOpen" class="searchable-dropdown-menu">
        <button
          v-for="option in filteredOptions"
          :key="getOptionKey(option)"
          type="button"
          class="searchable-dropdown-option"
          @click="selectOption(option)"
        >
          <slot name="option" :option="option">
            {{ getOptionLabel(option) }}
          </slot>
        </button>
        <div v-if="filteredOptions.length === 0" class="searchable-dropdown-empty">
          {{ noResultsText }}
        </div>
      </div>
      <input type="hidden" :value="modelValue" :required="required" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'

type SearchableOption = any

const props = withDefaults(
  defineProps<{
    modelValue: string
    options: SearchableOption[]
    placeholder?: string
    noResultsText?: string
    disabled?: boolean
    required?: boolean
    size?: 'sm' | 'md'
    containerClass?: string
    inputClass?: string
    getOptionKey: (option: any) => string
    getOptionLabel: (option: any) => string
    getOptionSearchText?: (option: any) => string
  }>(),
  {
    placeholder: 'Search...',
    noResultsText: 'No results found',
    disabled: false,
    required: false,
    size: 'md',
    containerClass: '',
    inputClass: '',
    getOptionSearchText: undefined,
  },
)

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'select', option: any): void
}>()

const rootRef = ref<HTMLElement | null>(null)
const isOpen = ref(false)
const searchTerm = ref('')

const selectedOption = computed(() => {
  return props.options.find(option => props.getOptionKey(option) === props.modelValue) || null
})

const sizeClass = computed(() => (props.size === 'sm' ? 'searchable-dropdown-sm' : ''))
const inputSizeClass = computed(() => (props.size === 'sm' ? 'form-input-sm' : ''))

watch(
  [selectedOption, () => props.options],
  () => {
    searchTerm.value = selectedOption.value ? props.getOptionLabel(selectedOption.value) : ''
  },
  { immediate: true },
)

const filteredOptions = computed(() => {
  const query = searchTerm.value.trim().toLowerCase()
  if (!query) return props.options

  return props.options.filter(option => {
    const text = (props.getOptionSearchText?.(option) ?? props.getOptionLabel(option)).toLowerCase()
    return text.includes(query)
  })
})

function openDropdown() {
  if (props.disabled) return
  isOpen.value = true
}

function closeDropdown() {
  isOpen.value = false
  searchTerm.value = selectedOption.value ? props.getOptionLabel(selectedOption.value) : ''
}

function handleInput() {
  isOpen.value = true
  if (selectedOption.value && searchTerm.value !== props.getOptionLabel(selectedOption.value)) {
    emit('update:modelValue', '')
  }
}

function selectOption(option: SearchableOption) {
  emit('update:modelValue', props.getOptionKey(option))
  emit('select', option)
  searchTerm.value = props.getOptionLabel(option)
  isOpen.value = false
}

function handleClickOutside(event: MouseEvent) {
  if (!rootRef.value?.contains(event.target as Node)) {
    closeDropdown()
  }
}

onMounted(() => {
  window.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  window.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.searchable-dropdown {
  width: 100%;
}

.searchable-dropdown-wrapper {
  position: relative;
}

.searchable-dropdown-display {
  width: 100%;
  min-height: 42px;
  padding: 10px 12px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  text-align: left;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: white;
  color: var(--text-primary);
  box-shadow: inset 0 0 0 1px transparent;
}

.searchable-dropdown-display:hover {
  border-color: var(--accent-primary);
}

.searchable-dropdown-display-text {
  flex: 1;
  min-width: 0;
  white-space: normal;
  overflow-wrap: anywhere;
  line-height: 1.35;
}

.searchable-dropdown-display-text.is-placeholder {
  color: var(--text-secondary);
}

.searchable-dropdown-wrapper .form-input {
  width: 100%;
  padding-right: 32px;
}

.searchable-dropdown-arrow {
  color: var(--text-secondary);
  font-size: 0.72rem;
  cursor: pointer;
  user-select: none;
  flex: 0 0 auto;
}

.searchable-dropdown-menu {
  position: absolute;
  left: 0;
  right: 0;
  top: calc(100% + 4px);
  z-index: 50;
  max-height: 220px;
  overflow-y: auto;
  width: max(100%, 320px);
  max-width: min(520px, calc(100vw - 32px));
  background: white;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: var(--shadow-md);
}

.searchable-dropdown-option {
  width: 100%;
  text-align: left;
  display: block;
  padding: 10px 12px;
  background: white;
  border: 0;
  border-bottom: 1px solid var(--border-color);
  color: var(--text-primary);
  cursor: pointer;
  white-space: normal;
  line-height: 1.35;
}

.searchable-dropdown-option:hover {
  background: var(--accent-primary-bg);
}

.searchable-dropdown-option:last-child {
  border-bottom: 0;
}

.searchable-dropdown-empty {
  padding: 10px 12px;
  color: var(--text-secondary);
  font-size: 0.9rem;
}

.searchable-dropdown-option :slotted(*) {
  display: block;
}

.searchable-dropdown-sm .searchable-dropdown-menu {
  max-height: 180px;
}
</style>
