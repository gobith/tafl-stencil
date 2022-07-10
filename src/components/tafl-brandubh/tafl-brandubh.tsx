import { Component, h, State } from '@stencil/core';
import init , { Brandubh } from "../../../wasm/pkg/wasm";

@Component({
  tag: 'tafl-brandubh',
  styleUrl: 'tafl-brandubh.css',
  shadow: true,
})
export class TaflBrandubh {
  brandubh
  pieceIndex: number;

  @State() board: Array<number> = [];


  async componentDidLoad() {
    await init();
    this.brandubh = new Brandubh();
    this.board = Array.from(this.brandubh.board());
   
  }

  pieceClicked = (event: any , index: number) => {
    event.stopPropagation();
    this.pieceIndex = index;
    console.log('piece clicked', index);
  };

  tileClicked = (tileIndex: number) => {
    console.log('tile clicked', this.pieceIndex , tileIndex);
    this.brandubh.move_piece(this.pieceIndex , tileIndex);
    this.board = Array.from(this.brandubh.board());
  };

  render() {
    return (
      <div class="container">
        {this.board.map((element, index) => {
          const cls = 'tile tile-' + element;
          let piece = <div></div>;
          switch (element) {
            case 3:
              piece = (
                <span
                  class="piece king"
                  onClick={(event) => {
                    this.pieceClicked(event , index);
                  }}
                ></span>
              );
              break;
            case 1:
              piece = (
                <span
                  class="piece attacker"
                  onClick={(event) => {
                    this.pieceClicked(event , index);
                  }}
                ></span>
              );
              break;
            case 2:
              piece = (
                <span
                  class="piece defender"
                  onClick={(event) => {
                    this.pieceClicked(event , index);
                  }}
                ></span>
              );
              break;
          }

          return (
            <div
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
}
