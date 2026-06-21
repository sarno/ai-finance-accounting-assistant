<template>
  <div v-if="isOpen" class="modal-overlay" style="z-index: 1100;" @click.self="$emit('cancel')">
    <transition name="modal-scale" appear>
      <div class="modal-content confirm-modal-content">
        <div class="modal-header">
          <h3>{{ title }}</h3>
          <button class="modal-close" @click="$emit('cancel')">&times;</button>
        </div>
        <div class="modal-body confirm-modal-body">
          <div class="confirm-modal-icon-wrapper" :class="{ 'is-danger': isDanger }">
            <span v-if="isDanger" class="confirm-icon">⚠️</span>
            <span v-else class="confirm-icon">❓</span>
          </div>
          <p class="confirm-modal-message">{{ message }}</p>
        </div>
        <div class="modal-footer confirm-modal-footer">
          <button type="button" class="btn btn-secondary btn-sm" @click="$emit('cancel')">
            {{ cancelText }}
          </button>
          <button
            type="button"
            :class="['btn btn-sm', isDanger ? 'btn-danger' : 'btn-primary']"
            @click="$emit('confirm')"
          >
            {{ confirmText }}
          </button>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
defineProps({
  isOpen: {
    type: Boolean,
    required: true
  },
  title: {
    type: String,
    default: 'Confirm Action'
  },
  message: {
    type: String,
    required: true
  },
  confirmText: {
    type: String,
    default: 'Confirm'
  },
  cancelText: {
    type: String,
    default: 'Cancel'
  },
  isDanger: {
    type: Boolean,
    default: false
  }
})

defineEmits(['confirm', 'cancel'])
</script>

<style scoped>
.confirm-modal-content {
  max-width: 440px !important;
}

.confirm-modal-body {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 30px 24px !important;
  gap: 16px;
}

.confirm-modal-icon-wrapper {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  background-color: var(--accent-primary-glow);
  display: flex;
  align-items: center;
  justify-content: center;
}

.confirm-modal-icon-wrapper.is-danger {
  background-color: var(--danger-bg);
}

.confirm-icon {
  font-size: 1.8rem;
}

.confirm-modal-message {
  font-size: 0.95rem;
  color: var(--text-secondary);
  line-height: 1.5;
}

.confirm-modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 16px 24px;
  border-top: 1px solid var(--border-color);
}

.modal-scale-enter-active,
.modal-scale-leave-active {
  transition: all 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.modal-scale-enter-from,
.modal-scale-leave-to {
  opacity: 0;
  transform: scale(0.9) translateY(10px);
}
</style>
