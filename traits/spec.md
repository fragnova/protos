# Traits
#trait #traits #edn #shards

*tl;dr*
Traits are the foundation for general and generic interoperability, as well as forward compatible code evolution.

## Trait Declaration
```clojure
(trait example
       :shards1
         {:Type Type.Shards
          :Requires ["variable1" Type.Int]
          :Inputs [Type.Int]
          :Output Type.Int}
       :int1 [Type.None Type.Int]
       :float1 Type.Float)
```

### Things to notice
Instead of `[Type.None]` a single type can be defined in a simplified way without the vector, like `Type.None`.
#### *shards1*
This is an example of how we define more complex entries.
Instead of a simple `Type.` definition we use a map `{}` to define the complex characteristics of the shards flow.
Notice that a wire can have different types of `Inputs` but a single type of `Output`.
## Trait Definition
```clojure
(deftrait example
          :shards1 (-> (Math.Add .variable1))
          :int1 10
          :float1 22.2)
```
What `deftrait` does internally is:
* In the current scripting (mal) `environment` it will insert `example` as a map containing those keywords (`:shards1`, etc.).

Basically other code running after will find the trait `example` as a map and can access the contract items.

E.g.
```clojure
(defwire compute
         11 = .variable1
         10
         (get example :shards1)
         (Assert.Is 21)
         (Math.Add (get example :int1))
         (Assert.Is 31))
```

**TODO**
We need to validate that `deftrait` and the defined items actually follow 1:1 the `trait` specification.

## More concrete examples
### A weapon
#### Damaging trait
```clojure
(trait Damaging
       ; a pure value would be hard to express in a interoperable way
       :damage-multiplier {:Type Type.Float :Min 0.0 :Max 1.0})
```

#### Weapon script definition
```clojure
(deftrait Damaging
          :damage-multiplier 0.1)

...
```

#### A game able to load `Damaging` trait weapons
```clojure
...

(defwire logic
         ...

         max-damage (Math.Multiply (get Damaging :damage-multiplier))
         ...)

...
```

### Discord Bot
#### Trait for a custom bot behavior
```clojure
(trait Discord-Bot
       :token Type.String
       :on-message-create [Type.None {:Type Type.Shards :Inputs Type.Table}])
```
#### Actual code
```clojure
(def bot-token (get Discord-Bot :token))

(def discord-dispatch
  (->
   (Take "t") (ExpectString)
   (Match ["MESSAGE_CREATE" (get Discord-Bot :on-message-create)
           nil nil])))

(def discord-gateway "gateway.discord.gg")
(def discord-gateway-target "/?v=6&encoding=json")

; gateway opcodes
(def op-identify 2)
(def op-heartbeat 1)
(def op-hello 10)
(def op-heartbeat-ack 11)
(def op-dispatch 0)

(defloop discord-bot-reader
  (WebSocket.ReadString .ws-discord-bot) (FromJson) (ExpectTable) = .payload
  (Take "op") (ExpectInt)
  (Match [op-heartbeat-ack nil
          op-dispatch (-> .payload discord-dispatch)
          nil (-> .payload (Log "unknown op"))]))

(defloop heartbeat
  {"op" op-heartbeat "d" .discord-session} (ToJson)
  (WebSocket.WriteString .ws-discord-bot)
  (Pause 4.0))

(defloop discord-bot-loop
  (Setup
   (WebSocket.Client "ws-discord-bot" discord-gateway
                     :Target discord-gateway-target :Secure true :Port 443)
   ; receive and handle hello
   (WebSocket.ReadString .ws-discord-bot) (FromJson) (ExpectTable)
   (Log "Hello payload")
   (Take "op") (Assert.Is op-hello) ; ensure it's a hello
   ; craft our hello
   {"op" op-identify
    "d" {"token" bot-token
         "properties" {"$os" "any"
                       "$browser" "shards"
                       "$device" "unknown"}
         "compress" false}}
   (ToJson) (Log "Sending log-in") (WebSocket.WriteString .ws-discord-bot)
   (WebSocket.ReadString .ws-discord-bot) (Log "Login response")
   (FromJson) (ExpectTable)
   (| (Take "t") (Assert.Is "READY"))
   (| (Take "s") = .discord-session))
  (Branch [heartbeat discord-bot-reader]))

(defmesh root)
(schedule root discord-bot-loop)
(run root 0.1)
```