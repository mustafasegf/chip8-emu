L200: CALL L2F6               ; 22F6
      LD   VB, #0C            ; 6B0C
      LD   VC, #3F            ; 6C3F
      LD   VD, #0C            ; 6D0C
      LD   I,  #2EA           ; A2EA
      DRW  VA, VB, #6         ; DAB6
      DRW  VC, VD, #6         ; DCD6
      LD   VE, #00            ; 6E00
      CALL L2D4               ; 22D4
      LD   V6, #03            ; 6603
      LD   V8, #02            ; 6802
L216: LD   V0, #60            ; 6060
      LD   DT, V0             ; F015
L21A: LD   V0, DT             ; F007
      SE   V0, #00            ; 3000
      JP   L21A               ; 121A
      RND  V7, #17            ; C717
      ADD  V7, #08            ; 7708
      LD   V9, #FF            ; 69FF
      LD   I,  #2F0           ; A2F0
      DRW  V6, V7, #1         ; D671
L22A: LD   I,  #2EA           ; A2EA
      DRW  VA, VB, #6         ; DAB6
      DRW  VC, VD, #6         ; DCD6
      LD   V0, #01            ; 6001
      SKNP V0                 ; E0A1
      ADD  VB, #FE            ; 7BFE
      LD   V0, #04            ; 6004
      SKNP V0                 ; E0A1
      ADD  VB, #02            ; 7B02
      LD   V0, #1F            ; 601F
      AND  VB, V0             ; 8B02
      DRW  VA, VB, #6         ; DAB6
      LD   V0, #0C            ; 600C
      SKNP V0                 ; E0A1
      ADD  VD, #FE            ; 7DFE
      LD   V0, #0D            ; 600D
      SKNP V0                 ; E0A1
      ADD  VD, #02            ; 7D02
      LD   V0, #1F            ; 601F
      AND  VD, V0             ; 8D02
      DRW  VC, VD, #6         ; DCD6
      LD   I,  #2F0           ; A2F0
      DRW  V6, V7, #1         ; D671
      ADD  V6, V8             ; 8684
      ADD  V7, V9             ; 8794
      LD   V0, #3F            ; 603F
      AND  V6, V0             ; 8602
      LD   V1, #1F            ; 611F
      AND  V7, V1             ; 8712
      SNE  V6, #00            ; 4600
      JP   L278               ; 1278
      SNE  V6, #3F            ; 463F
      JP   L282               ; 1282
L26C: SNE  V7, #1F            ; 471F
      LD   V9, #FF            ; 69FF
      SNE  V7, #00            ; 4700
      LD   V9, #01            ; 6901
      DRW  V6, V7, #1         ; D671
      JP   L22A               ; 122A
L278: LD   V8, #02            ; 6802
      LD   V3, #01            ; 6301
      LD   V0, V7             ; 8070
      SUB  V0, VB             ; 80B5
      JP   L28A               ; 128A
L282: LD   V8, #FE            ; 68FE
      LD   V3, #0A            ; 630A
      LD   V0, V7             ; 8070
      SUB  V0, VD             ; 80D5
L28A: SE   VF, #01            ; 3F01
      JP   L2A2               ; 12A2
      LD   V1, #02            ; 6102
      SUB  V0, V1             ; 8015
      SE   VF, #01            ; 3F01
      JP   L2BA               ; 12BA
      SUB  V0, V1             ; 8015
      SE   VF, #01            ; 3F01
      JP   L2C8               ; 12C8
      SUB  V0, V1             ; 8015
      SE   VF, #01            ; 3F01
      JP   L2C2               ; 12C2
L2A2: LD   V0, #20            ; 6020
      LD   ST, V0             ; F018
      CALL L2D4               ; 22D4
      ADD  VE, V3             ; 8E34
      CALL L2D4               ; 22D4
      LD   V6, #3E            ; 663E
      SE   V3, #01            ; 3301
      LD   V6, #03            ; 6603
      LD   V8, #FE            ; 68FE
      SE   V3, #01            ; 3301
      LD   V8, #02            ; 6802
      JP   L216               ; 1216
L2BA: ADD  V9, #FF            ; 79FF
      SNE  V9, #FE            ; 49FE
      LD   V9, #FF            ; 69FF
      JP   L2C8               ; 12C8
L2C2: ADD  V9, #01            ; 7901
      SNE  V9, #02            ; 4902
      LD   V9, #01            ; 6901
L2C8: LD   V0, #04            ; 6004
      LD   ST, V0             ; F018
      ADD  V6, #01            ; 7601
      SNE  V6, #40            ; 4640
      ADD  V6, #FE            ; 76FE
      JP   L26C               ; 126C
L2D4: LD   I,  #2F2           ; A2F2
      LD   B,  VE             ; FE33
      LD   V2, [I]            ; F265
      LD   F,  V1             ; F129
      LD   V4, #14            ; 6414
      LD   V5, #00            ; 6500
      DRW  V4, V5, #5         ; D455
      ADD  V4, #15            ; 7415
      LD   F,  V2             ; F229
      DRW  V4, V5, #5         ; D455
      RET                     ; 00EE
L2EA: db #80, #80, #80, #80
L2EE: db #80, #80, #80, #00
L2F2: db #00, #00, #00, #00
L2F6: LD   VB, #20            ; 6B20
      LD   VC, #00            ; 6C00
      LD   I,  #2EA           ; A2EA
L2FC: DRW  VB, VC, #1         ; DBC1
      ADD  VC, #01            ; 7C01
      SE   VC, #20            ; 3C20
      JP   L2FC               ; 12FC
      LD   VA, #00            ; 6A00
      RET                     ; 00EE
