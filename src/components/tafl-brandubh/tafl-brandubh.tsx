import { Component, h, State, Element } from '@stencil/core';
import init, { Brandubh } from '../../../wasm/pkg/wasm';

@Component({
  tag: 'tafl-brandubh',
  styleUrl: 'tafl-brandubh.css',
  shadow: true,
})
export class TaflBrandubh {
  brandubh;
  pieceIndex: number;

  @Element() private element: HTMLElement;
  @State() board: Array<number> = [];
  @State() playerSide: string = 'defender';
  @State() gameStatus: string = 'start';

  async componentDidLoad() {
    await init();
    this.startNewGame();
  }


  startNewGame() {
    this.gameStatus = 'start';
    if (this.brandubh) {this.brandubh.free()};
    this.brandubh = new Brandubh();
    this.board = Array.from(this.brandubh.board());

  }

  pieceClicked = (event: any, index: number) => {
    event.stopPropagation();
    console.log(this.gameStatus);
    if (this.gameStatus !== 'play') {
      return;
    }
    this.pieceIndex = index;
    console.log('piece clicked', index);
  };

  tileClicked = (tileIndex: number) => {
    if (this.gameStatus !== 'play') {
      return;
    }
    console.log('tile clicked', this.pieceIndex, tileIndex);
    let status = this.brandubh.move_piece(this.pieceIndex, tileIndex);

    if (status === 0) {
      return;
    }

    this.animateMove(this.pieceIndex, tileIndex);

    console.log('from rust', this.brandubh.get_string());

    const promise = new Promise(resolve => setTimeout(resolve, 500));
    promise.then(() => {
      this.board = Array.from(this.brandubh.board());
    });
  };

  animateMove = (startIndex: number, endIndex: number) => {
    const startX = (startIndex % 7) + 1;
    const startY = Math.floor(startIndex / 7) + 1;
    const endX = (endIndex % 7) + 1;
    const endY = Math.floor(endIndex / 7) + 1;

    const x = (endX - startX) * 30;
    const y = (endY - startY) * 30;

    const piece = this.element.shadowRoot.querySelector('#tile-' + startIndex).firstChild as HTMLElement;

    piece.style.transform = `translate(${x}px , ${y}px)`;
  };

  render() {
    return (
      <div class="container">
        {this.renderBoard()}
        {this.renderInfo()}
      </div>
    );
  }

  renderBoard() {
    return (
      <div class="board">
        {this.board.map((element, index) => {
          const cls = 'tile tile-' + element;
          const id = 'tile-' + index;
          let piece = <div></div>;
          switch (element) {
            case 3:
              piece = (
                <span
                  class="piece king"
                  onClick={event => {
                    this.pieceClicked(event, index);
                  }}
                ></span>
              );
              break;
            case 1:
              piece = (
                <span
                  class="piece attacker"
                  onClick={event => {
                    this.pieceClicked(event, index);
                  }}
                ></span>
              );
              break;
            case 2:
              piece = (
                <span
                  class="piece defender"
                  onClick={event => {
                    this.pieceClicked(event, index);
                  }}
                ></span>
              );
              break;
          }

          return (
            <div
              id={id}
              class={cls}
              onClick={() => {
                this.tileClicked(index);
              }}
            >
              {piece}
            </div>
          );
        })}
      </div>
    );
  }

  renderInfo() {
    return <div class="info">{this.gameStatus === 'start' ? this.startProcedure() : this.playProcedure()} </div>;
  }

  startProcedure() {
    let defenderClass = '';
    let attackerClass = '';

    if (this.playerSide === 'defender') {
      defenderClass = 'selected-side';
    } else {
      attackerClass = 'selected-side';
    }

    return (
      <div>
        <div>
          <button
            class={defenderClass}
            onClick={() => {
              this.playerSide = 'defender';
              console.log(this.playerSide);
            }}
          >
            Defender
          </button>
          <button
            class={attackerClass}
            onClick={() => {
              this.playerSide = 'attacker';
              console.log(this.playerSide);
            }}
          >
            Attacker
          </button>
        </div>
        <button
          onClick={() => {
            this.gameStatus = 'play';
          }}
        >
          Start
        </button>
      </div>
    );
  }

  playProcedure() {
    return (
      <div>
        <button
          onClick={() => {
            this.startNewGame();
          }}
        >
          New Game
        </button>
      </div>
    );
  }
}
