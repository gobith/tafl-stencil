import { Component, Host, h, State } from '@stencil/core';

@Component({
  tag: 'tafl-brandubh',
  styleUrl: 'tafl-brandubh.css',
  shadow: true,
})
export class TaflBrandubh {
  @State() board: Array<number> = [
    4, 0, 3, 3, 3, 0, 4, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 2, 0, 0, 3, 3, 3, 2, 5, 2, 3, 3, 3, 0, 0, 2, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 4, 0, 3, 3, 3, 0, 4,
  ];

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
            case 5:
              piece = (
                <span
                  class="piece king"
                  onClick={(event) => {
                    this.pieceClicked(event , index);
                  }}
                ></span>
              );
              break;
            case 3:
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
