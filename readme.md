# GOMOKU
간단히 오목을 두는 사이트
url: https://gogomoku.vercel.app/ 
 
#### 사용 스택
프론트엔드: 넥스트js
프론트엔드배포: 버셀
백엔드: 악섬, 람다런타임, 다이나모디비
백엔드배포: API는 람다, 웹소켓은 AWS GW 웹소켓

---
# DDB

## USER

|PK                 |SK                 |ATTR_NAME  |ATTR_TYPE  |
|-                  |-                  |-          |-          |
|`USER#<nanoid>`    |`INFO`             |createdAt  |timestamp  |
|                   |                   |nickName   |string     |
|                   |                   |type       |string     | guest / google / email
|                   |                   |status     |string     | ok / 
|                   |                   |role       |string     | user / admin

## OAUTH2

|PK                 |SK                         |ATTR_NAME  |ATTR_TYPE  |
|-                  |-                          |-          |-          |
|`OAUTH2#<type>`    |`ID#<str>`                 |userId     |nanoid     |
|                   |                           |etc        |map        |

## SESSION
|PK                 |SK             |ATTR_NAME  |ATTR_TYPE  |
|-                  |-              |-          |-          |
|`SESSION#<str>`    |`INFO`         |jwt        |str        |
|                   |               |userId     |nanoid     |

## CHANNEL
|PK                 |SK             |ATTR_NAME  |ATTR_TYPE  |
|-                  |-              |-          |-          |
|`CHANNEL#<str>`    |`ROOM#<num>`   |roomId     |nanoid     |
|                   |               |roomNum    |number     |

## ROOM
|PK             |SK                     |ATTR_NAME      |ATTR_TYPE  |
|-              |-                      |-              |-          |
|`ROOM#<nanoid>`|`INFO`                 |playerBlack    |nanoid     |
|               |                       |playerWhite    |nanoid     |
|               |                       |channel        |string     |
|               |                       |roomName       |string     |
|               |                       |roomId         |string     |
|               |`HST#<timestamp>`      |type           |string     |
|               |                       |event          |map        |

## WS_CONN
|PK                 |SK                 |ATTR_NAME  |ATTR_TYPE  |
|-                  |-                  |-          |-          |
|`WS_CONN#<str>`    |`INFO`             |createdAt  |timestamp  |
|                   |                   |wsToken    |string?    |
|                   |                   |userId     |nanoid?    |
|                   |`WS_TOPIC#<str>`   |userId     |nanoid     |
|`USER#<nanoid>`    |`WS_CONN#<str>`    |createdAt  |timestamp  |

## WS_TOPIC
|PK                 |SK                 |ATTR_NAME  |ATTR_TYPE  |
|-                  |-                  |-          |-          |
|`WS_TOPIC#<str>`   |`WS_CONN#<str>`    |userId     |nanoid     |


---
# WS


## 초기
```json
{
    "t": "wsInitial"
    "d": {
        "jwt": "<string?>"
    }
}
```
```json
{
    "t": "wsInitialRes"
}
```

## 토픽 설정
```json
{
    "t": "setTopic"
    "d": {
        "topic": "<string>"
    }
}
```
```json
{
    "t": "setTopicRes"
}
```

## jwt 세팅

```json
{
    "t": "setJwt"
    "d": {
        "jwt": "<string>"
    }
}
```
```json
{
    "t": "setJwtRes"
}
```

WS 인증 프로세스

case1: session token 이존재하는상황.
1. jwt refrsh api 쏨 이후 jwt memory 저장
2. ws 연결
3. ws 연결 일단 첫 메시지로, wsInitial를 올려보냄
4. 그러면 WS_CONN을 에다가 데이터 넣음(올라온 jwt로 info 쏴서)
5. 이후 InitialEnd 보내줌
6. 이후 클라에서 현재 토픽에 맞는 데이터 요청함
   이때 rest 쏠때 jwt가필요한데 이거는 WS_CONN을 뒤져서 jwt를 가져와서 사용한다.


WS REQ TYPE

//초기 연결용
ty: wsInitial
jwt: Option<String>
topic: String

// 토픽 세팅할때
ty: setTopic
topic: string

// accesstoekn 리프레시 
ty: tokenRefresh
jwt: string


//방 목록 가져오기
ty: fetchRoomList
rooms: [{}]

ty: fetchRoom


WS RES TYPE




WS CONN

$connect: 아무것도 안함.
    연결이후 클라이언트에서 첫메시지로: wsInitial 보내줌
    그러면 WS_CONN, WS_TOPIC을 세팅해줌
    wsInitial에 jwt가 있다면 거기서 id를꺼내서 WS_CONN에 USER 세팅도 해준다.

$disconnect: 연결 해제
    disconnect api를 쏴서
    WS_CONN, WS_TOPIC을 지워줌




ROOM HST: 채팅, 플레이어 이동, 세팅 변경
MATCH HST: 돌놓기

HOME 진입
서버에 쏴서 액세스토큰 발급
WS 연결,
    Auth 날려서 jwt 보내고 ddb엣 CONID#USERID 저장
    
    
    현재 url을 보고 데이터 받을 거 정함
    1. HOME
    2. ROOM
   
ROOM 진입
ROOM에서는 USERID가 없으면 홈으로 보낸다.

----

홈에서는 로그인 하지 않아도 룸 목록을 볼수있다.
채널은 일단 1로 고정

게스트 로그인: 닉네임만으로 userid 발급 추후 소셜로그인 연동가능

todo
소셜 로그인: 소셜로 가입

todo
게스트 -> 소셜 전환

게스트 
액세스토큰 1일
세션토큰 무한

가입자
액세스토큰 1일
세션토큰 14일


---

ws가 끊어지면 reconnect 화면으로 전환



---
ws는 연결핸들링용, 데이터는 api에서
그러기위해 ws전에 jwt 발급 받아놔야함
---

ws
세션 token 검증 
방 만들기


