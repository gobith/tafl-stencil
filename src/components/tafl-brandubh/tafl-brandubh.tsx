import { Component, Host, h, State } from '@stencil/core';
import init , { Size } from "../../../wasm/pkg/wasm";

@Component({
  tag: 'tafl-brandubh',
  styleUrl: 'tafl-brandubh.css',
  shadow: true,
})
export class TaflBrandubh {
  size
  @State() board: Array<number> = [];


  async componentDidLoad() {
    await init();
    this.size = new Size(1 , 2);
    this.board = Array.from(this.size.contents());
   
  }

  pieceClicked = (event: any , index: number) => {
    event.stopPropagation();
    console.log('piece clicked', index);
  };

  tileClicked = (index: number) => {
    console.log('tile clicked', index);
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
