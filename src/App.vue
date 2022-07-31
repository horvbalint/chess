<template>
  <div class="wrapper">
    <dead-pieces
      :pieces="board.deadBlacks"
      :char-dict="pieceChars"
      :size="board.tileSize"
    />

    <canvas ref="board"/>
    
    <dead-pieces
      :pieces="board.deadWhites"
      :char-dict="pieceChars"
      :size="board.tileSize"
    />
  </div>
</template>

<script>
import DeadPieces from './components/dead-pieces.vue'
import {invoke} from '@tauri-apps/api'

export default {
  data() {
    return {
      board: {
        size: 0,
        tileSize: 0,
        state: null,
        deadWhites: [],
        deadBlacks: [],
        currPlayer: 'White'
      },
      possibleSteps: [],
      ctx: null,
      pieceChars: {
        Pawn: {White: 'p', Black: 'o'},
        Rook: {White: 'r', Black: 't'},
        Knight: {White: 'n', Black: 'm'},
        Bishop: {White: 'b', Black: 'v'},
        King: {White: 'k', Black: 'l'},
        Queen: {White: 'q', Black: 'w'},
      }
    }
  },
  mounted() {
    window.addEventListener('beforeunload', () => invoke('reset'))
    window.addEventListener('resize', () => {
      this.setUpBoard()
      this.drawBoard()
    })

    this.$refs.board.addEventListener('click', ({offsetX, offsetY}) => {
      offsetX = this.board.currPlayer == 'White' ? offsetX : this.$refs.board.width - offsetX
      offsetY = this.board.currPlayer == 'White' ? offsetY : this.$refs.board.height - offsetY

      let x = Math.floor(offsetX / this.board.tileSize)
      let y = Math.floor(offsetY / this.board.tileSize)

      if(this.board.selectedTile) this.tryStep(x, y)
      else this.selectTile(x, y)
    })

    this.setUpBoard()

    Promise.all([
      invoke('get_state'),
      document.fonts.ready,
    ])
      .then( ([state]) => {
        this.board.state = state
        this.drawBoard()
      })
      .catch( err => console.error(err) )
  },
  methods: {
    setUpBoard() {
      this.board.size = Math.min(window.innerWidth, window.innerHeight)
      this.board.tileSize = this.board.size / 8

      this.$refs.board.width = this.board.size
      this.$refs.board.height = this.board.size

      this.ctx = this.$refs.board.getContext('2d')
      this.ctx.font = `${this.board.tileSize}px Chess`
    },
    selectTile(x, y) {
      if(!this.board.state[y][x] || this.board.state[y][x].color !== this.board.currPlayer) return
      this.board.selectedTile = {x, y}
    
      invoke('get_steps', {pos: {x, y}})
        .then( steps => {
          if(!steps) return

          this.possibleSteps = steps
          this.drawBoard()
        })
    },
    tryStep(x, y) {
      let fromX = this.board.selectedTile.x
      let fromY = this.board.selectedTile.y

      if(x == fromX && y == fromY) {
        this.board.selectedTile = null
        this.drawBoard()
        return
      }

      if(this.possibleSteps.every(step => step.x != x || step.y != y)) return

      invoke('move_piece', {
        from: {x: fromX, y: fromY},
        to: {x, y},
      })

      if(this.board.state[y][x]) {
        if(this.board.state[y][x].color == 'White')
          this.board.deadWhites.unshift(this.board.state[y][x])
        else
          this.board.deadBlacks.unshift(this.board.state[y][x])
      }

      this.board.state[y][x] = this.board.state[fromY][fromX]
      this.board.state[fromY][fromX] = null

      this.board.selectedTile = null
      this.possibleSteps = []

      console.log('HERE')
      this.board.currPlayer = this.board.currPlayer == 'White' ? 'Black' : 'White'

      this.drawBoard()
    },
    drawBoard() {
      let rotation = this.board.currPlayer == 'White' ? 0 : Math.PI
      this.rotateCtx(rotation)
      
      this.ctx.fillStyle = '#ffffff'
      this.ctx.fillRect(0, 0, this.board.size, this.board.size)

      for(let i=0; i<this.board.state.length; ++i) {
        for(let j=0; j<this.board.state.length; ++j) {
          let x = j * this.board.tileSize
          let y = i * this.board.tileSize

          if((i+j) % 2 == 1) {
            this.ctx.fillStyle = '#999999'
            this.ctx.fillRect(x, y, this.board.tileSize, this.board.tileSize)
          }

          if(this.board.selectedTile) {
            if(this.possibleSteps.find(({x, y}) => x == j && y == i)) {
              this.ctx.fillStyle = 'rgba(0, 0, 255, 0.3)'
              this.ctx.fillRect(x, y, this.board.tileSize, this.board.tileSize)
            }

            if(this.board.selectedTile.x == j && this.board.selectedTile.y == i) {
              this.ctx.fillStyle = '#00ff28'
              this.ctx.fillRect(x, y, this.board.tileSize, this.board.tileSize)
            }
          } 

          let piece = this.board.state[i][j]
          if(piece) {
            this.ctx.save()
            this.rotateCtx(rotation, x + this.board.tileSize/2, y + this.board.tileSize/2)
            this.ctx.fillStyle = '#000000'
            this.ctx.textBaseline = 'top'

            let pieceChar = this.pieceChars[piece.rank][piece.color]
            this.ctx.fillText(pieceChar, x, y)
            
            this.ctx.restore()
          }
        }
      }

      this.rotateCtx(-rotation)
    },
    rotateCtx(angle, transX = this.$refs.board.width/2, transY = this.$refs.board.height/2) {
      this.ctx.translate(transX, transY)
      this.ctx.rotate(angle)
      this.ctx.translate(-transX, -transY)
    }
  },
  components: {
    DeadPieces
  }
}
</script>

<style scoped>
.wrapper {
  height: 100vh;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: center;
  gap: 10px;
}
canvas {
  box-shadow: 0 0 30px black;
  flex-shrink: 0;
}
</style>
