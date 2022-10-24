setupBoard = function() {
    
    var board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    var board = document.getElementById("chess-board");
    
    for (var row = 0; row < 8; row++) {
    
        var rank = document.createElement("div");
        rank.className = "rank";
        board.appendChild(rank);
    
        for (var col = 0; col < 8; col++) {
    
            var square = document.createElement("div");
            square.className = "board-square";
    
            if ((row + col) % 2 == 0) {
                square.style.backgroundColor = "wheat";
            } else {
                square.style.backgroundColor = "darkblue";
            }
            
            rank.appendChild(square);
    
        }
    }
    
    var row = 0;
    var col = 0;
    
    var addPiece = function(row, col, src) {
        var img = document.createElement("img");
        img.className = "chess-piece";
        img.src = "images/" + src + ".png";
        board.children[row].children[col].appendChild(img);   
    }
    
    var done = false;
    
    for (var char = 0; char < board_fen.length; char++) {
    
        if (done) {
            break;
        }
    
        switch (board_fen[char]) {
            case "P":
                addPiece(row, col, "white_pawn");
                break;
            case "p":
                addPiece(row, col, "black_pawn");
                break;
            case "N":
                addPiece(row, col, "white_knight");
                break;
            case "n":
                addPiece(row, col, "black_knight");
                break;
            case "B":
                addPiece(row, col, "white_bishop");
                break;
            case "b":
                addPiece(row, col, "black_bishop");
                break;
            case "R":
                addPiece(row, col, "white_rook");
                break;
            case "r":
                addPiece(row, col, "black_rook");
                break;
            case "Q":
                addPiece(row, col, "white_queen");
                break;
            case "q":
                addPiece(row, col, "black_queen");
                break;
            case "K":
                addPiece(row, col, "white_king");
                break;
            case "k":
                addPiece(row, col, "black_king");
                break;
            case "/":
                row += 1;
                col = -1;
                break;
            case " ":
                done = true;
                break;
            default:
                col += board_fen[char] - 1;
        }
        col += 1;
    }

}