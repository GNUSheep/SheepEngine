var game = new Chess()

function onDragStart(square, piece) {
    var moves = game.moves({
        square: square,
        verbose: true
    })
    
    moves.forEach(square => {
        var square_obj = $('#chess_board .square-' + square.to)
        square_obj.addClass('highlight1-32417')
    });
}

function onDrop(source, target) {
    var move = game.move({
        from: source,
        to: target,
        promotion: 'q'
    })
    
    if (move === null) {    
        return 'snapback'
    }

    $.post("http://localhost:8080", game.fen(), (data, status) => {
        console.log(status)
        let from = data.slice(0, 2)
        let to = data.slice(2)

        game.move({
            from: from,
            to: to,
            promotion: 'q'
        })
    });
}

function onSnapEnd () {
    chess_board.position(game.fen())
}

var config = {
    draggable: true,
    onDragStart: onDragStart,
    onDrop: onDrop,
    onSnapEnd: onSnapEnd,
    position: 'start',
}

var chess_board = Chessboard('chess_board', config)
$(window).resize(chess_board.resize)