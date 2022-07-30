<template>
  <div class="pieces-wrapper" :style="{width: `${pieceSize}px`}">
    <p
      v-for="(piece, index) of pieces"
      :key="index"
      :style="{fontSize: `${pieceSize}px`}"
    >
      {{charDict[piece.rank][piece.color]}}
    </p>
  </div>
</template>

<script>
export default {
  props: {
    charDict: {
      type: Object,
      required: true
    },
    size: {
      type: Number,
      required: true
    },
    pieces: {
      type: Array,
      default: () => []
    },
  },
  computed: {
    pieceSize() {
      return this.$props.size * .7
    }
  }
}
</script>

<style scoped>
  .pieces-wrapper {
    height: 100%;
    overflow: auto;
  }
  p {
    font-family: Chess;
  }
</style>