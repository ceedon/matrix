module Matrix exposing (main)

--import Element.Border     as        Border
--import Element.Font       as        Font

import Browser
import Debug exposing (toString)
import Dict exposing (Dict)
import Element exposing (..)
import Element.Background as Background
import Element.Events exposing (onClick, onMouseEnter, onMouseLeave)
import Element.Region exposing (description)
import Html exposing (Html)
import Tuple exposing (first, second)
import Validate exposing (fromErrors, validate)


gridWidth : Int
gridWidth =
    8


gridHeight : Int
gridHeight =
    8




imageWidth : Float
imageWidth = 256

imageHeight : Float
imageHeight = 512

gridUnit : Float
gridUnit =
    imageWidth / 4


-- Model


type alias Model =
    { grid : Grid
    , cellsClicked : List Pos
    }


type alias Grid =
    Dict Pos CellColor


type alias Pos =
    ( Int, Int )


type CellColor
    = Black
    | Green
    | Red
    | Orange


ok : Model
ok =
    Model Dict.empty []



-- Init


init : () -> ( Model, Cmd Msg )
init _ =
    ( ok, Cmd.none )



-- View


view : Model -> Html Msg
view =
    printGrid


drawMatrix _ =
    Element.layout
        []
    <|
        text "Hello, World!"


printGrid : Model -> Html Msg
printGrid model =
    let
        grid =
            model.grid

        xs : List Int
        xs =
            List.range 1 gridWidth

        ys : List Int
        ys =
            List.range 1 gridHeight

        toRow : Int -> List Pos
        toRow y =
            List.map (\x -> ( x, y )) xs
    in
    layout [] <|
        column []
            [ column
                [ moveRight ((toFloat gridWidth + 1) * imageWidth / 2)
                , moveDown (imageHeight / 2)
                ]
                (List.map
                    (\y ->
                        drawCellRow grid (toRow y)
                    )
                    ys
                )
            ]


drawCell : Grid -> Pos -> Element Msg
drawCell _ p =
    el [] <|
        image
            [ moveDown (gridUnit * toFloat (first p))
            , moveLeft (2 * gridUnit * toFloat (first p))
            ]
            { src = "./img/block.png"
            , description = "Stolen block" ++ toString p
            }


drawCellRow : Grid -> List Pos -> Element Msg
drawCellRow b ps =
    let
        rowNum =
            case ps of
                p :: _ ->
                    toFloat(second p)

                [] ->
                    0.0
    in
    row
    [ moveUp ((imageHeight - gridUnit) * rowNum)
    , moveLeft (gridUnit * (2 * rowNum))] (List.map (drawCell b) ps)



-- Update


type Msg
    = ClickedCell Pos
    | ClickedDraw Grid


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ClickedCell p ->
            ( { model | cellsClicked = p :: model.cellsClicked }
            , Cmd.none
            )

        ClickedDraw g ->
            ( model, Cmd.none )



-- Subscriptions


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- Main


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        }
