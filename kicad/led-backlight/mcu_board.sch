EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 2 3
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L Device:CP1_Small C3
U 1 1 60E20FE9
P 5600 2800
F 0 "C3" H 5550 3000 50  0000 L CNN
F 1 "330uF" H 5450 2600 50  0000 L CNN
F 2 "Capacitor_Tantalum_SMD:CP_EIA-3528-12_Kemet-T" H 5600 2800 50  0001 C CNN
F 3 "~" H 5600 2800 50  0001 C CNN
F 4 "C178370" H 5600 2800 50  0001 C CNN "LCSC Part"
	1    5600 2800
	-1   0    0    1   
$EndComp
$Comp
L Device:CP1_Small C4
U 1 1 60E21649
P 6000 2800
F 0 "C4" H 5950 3000 50  0000 L CNN
F 1 "330uF" H 5850 2600 50  0000 L CNN
F 2 "Capacitor_Tantalum_SMD:CP_EIA-3528-12_Kemet-T" H 6000 2800 50  0001 C CNN
F 3 "~" H 6000 2800 50  0001 C CNN
F 4 "C178370" H 6000 2800 50  0001 C CNN "LCSC Part"
	1    6000 2800
	-1   0    0    1   
$EndComp
$Comp
L Device:CP1_Small C1
U 1 1 60E21AC7
P 3050 1900
F 0 "C1" H 3141 1946 50  0000 L CNN
F 1 "22uF" H 3141 1855 50  0000 L CNN
F 2 "Capacitor_Tantalum_SMD:CP_EIA-3528-12_Kemet-T" H 3050 1900 50  0001 C CNN
F 3 "~" H 3050 1900 50  0001 C CNN
F 4 "C8020" H 3050 1900 50  0001 C CNN "LCSC Part"
	1    3050 1900
	-1   0    0    1   
$EndComp
$Comp
L Device:C_Small C2
U 1 1 60E22C5D
P 4250 1900
F 0 "C2" H 4342 1946 50  0000 L CNN
F 1 "10uF" H 4342 1855 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 4250 1900 50  0001 C CNN
F 3 "~" H 4250 1900 50  0001 C CNN
F 4 "C15850" H 4250 1900 50  0001 C CNN "LCSC Part"
	1    4250 1900
	1    0    0    -1  
$EndComp
$Comp
L Regulator_Linear:AMS1117-3.3 U1
U 1 1 60E23CC9
P 3700 2250
F 0 "U1" H 3700 2492 50  0000 C CNN
F 1 "AMS1117-3.3" H 3700 2401 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-223-3_TabPin2" H 3700 2450 50  0001 C CNN
F 3 "http://www.advanced-monolithic.com/pdf/ds1117.pdf" H 3800 2000 50  0001 C CNN
F 4 "C6186" H 3700 2250 50  0001 C CNN "LCSC Part"
	1    3700 2250
	-1   0    0    1   
$EndComp
$Comp
L Transistor_FET:BSS138 Q1
U 1 1 60E25500
P 3700 3050
F 0 "Q1" V 3949 3050 50  0000 C CNN
F 1 "BSS138" V 4040 3050 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-23" H 3900 2975 50  0001 L CIN
F 3 "https://www.onsemi.com/pub/Collateral/BSS138-D.PDF" H 3700 3050 50  0001 L CNN
F 4 "C52895" H 3700 3050 50  0001 C CNN "LCSC Part"
	1    3700 3050
	0    1    1    0   
$EndComp
$Comp
L Device:R_Small_US R1
U 1 1 60E2A980
P 3050 2950
F 0 "R1" H 3118 2996 50  0000 L CNN
F 1 "10k" H 3118 2905 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" H 3050 2950 50  0001 C CNN
F 3 "~" H 3050 2950 50  0001 C CNN
F 4 "C17414" H 3050 2950 50  0001 C CNN "LCSC Part"
	1    3050 2950
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small_US R2
U 1 1 60E2AC66
P 4250 2950
F 0 "R2" H 4318 2996 50  0000 L CNN
F 1 "10k" H 4318 2905 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric" H 4250 2950 50  0001 C CNN
F 3 "~" H 4250 2950 50  0001 C CNN
F 4 "C17414" H 4250 2950 50  0001 C CNN "LCSC Part"
	1    4250 2950
	1    0    0    -1  
$EndComp
$Comp
L Seeeduino_XIAO:SeeeduinoXIAO U2
U 1 1 60E2C3D9
P 5900 4800
F 0 "U2" H 5875 3861 50  0000 C CNN
F 1 "SeeeduinoXIAO" H 5875 3770 50  0000 C CNN
F 2 "Seeeduino XIAO KICAD:Seeeduino XIAO-MOUDLE14P-2.54-21X17.8MM" H 5550 5000 50  0001 C CNN
F 3 "" H 5550 5000 50  0001 C CNN
F 4 "0" H 5900 4800 50  0001 C CNN "JLCPCB BOM"
	1    5900 4800
	1    0    0    -1  
$EndComp
Text HLabel 4200 4950 0    50   Input ~ 0
mode_btn
Text HLabel 5050 4650 0    50   Input ~ 0
mode_ref
$Comp
L power:GND #PWR03
U 1 1 60E30D73
P 6850 4500
F 0 "#PWR03" H 6850 4250 50  0001 C CNN
F 1 "GND" V 6855 4372 50  0000 R CNN
F 2 "" H 6850 4500 50  0001 C CNN
F 3 "" H 6850 4500 50  0001 C CNN
	1    6850 4500
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4200 4950 4700 4950
Wire Wire Line
	4700 4950 4700 5250
Wire Wire Line
	4700 5250 5050 5250
Wire Wire Line
	5050 4800 4700 4800
Wire Wire Line
	4700 4800 4700 4950
Connection ~ 4700 4950
NoConn ~ 5050 4350
NoConn ~ 5050 4500
NoConn ~ 5050 4950
NoConn ~ 5050 5100
NoConn ~ 5800 5650
NoConn ~ 6000 5650
NoConn ~ 6700 5250
NoConn ~ 6700 5100
NoConn ~ 6700 4650
NoConn ~ 6700 4950
Wire Wire Line
	3050 1800 3700 1800
Wire Wire Line
	3700 1950 3700 1800
Connection ~ 3700 1800
Wire Wire Line
	3700 1800 4250 1800
Wire Wire Line
	4250 2000 4250 2250
Wire Wire Line
	4250 2250 4000 2250
Wire Wire Line
	3050 2000 3050 2250
Wire Wire Line
	3050 2250 3400 2250
Wire Wire Line
	4250 2850 4250 2250
Connection ~ 4250 2250
Wire Wire Line
	4250 3050 4250 3150
Wire Wire Line
	3700 2850 3050 2850
Wire Wire Line
	3050 2250 3050 2850
Connection ~ 3050 2250
Connection ~ 3050 2850
Wire Wire Line
	3050 3150 3050 3050
Wire Wire Line
	2750 3150 3050 3150
Connection ~ 3050 3150
Wire Wire Line
	3050 3150 3500 3150
Wire Wire Line
	3900 3150 4250 3150
Connection ~ 4250 3150
Text Label 4600 3150 0    50   ~ 0
DataHigh
Text Label 2750 3150 2    50   ~ 0
DataLow
Wire Wire Line
	4250 2250 6900 2250
Wire Wire Line
	6700 4350 6900 4350
Wire Wire Line
	6700 4800 7250 4800
Text Label 7250 4800 0    50   ~ 0
DataLow
Connection ~ 6000 2900
Wire Wire Line
	6000 2900 5600 2900
Wire Wire Line
	5600 2700 6000 2700
Wire Wire Line
	6000 2700 6250 2700
Connection ~ 6000 2700
$Comp
L power:GND #PWR01
U 1 1 60E48CC9
P 6250 2700
F 0 "#PWR01" H 6250 2450 50  0001 C CNN
F 1 "GND" V 6255 2572 50  0000 R CNN
F 2 "" H 6250 2700 50  0001 C CNN
F 3 "" H 6250 2700 50  0001 C CNN
	1    6250 2700
	0    -1   -1   0   
$EndComp
Wire Wire Line
	6900 2250 6900 4350
Wire Wire Line
	5600 2900 5400 2900
Wire Wire Line
	5400 2900 5400 3300
Connection ~ 5600 2900
Wire Wire Line
	5500 3150 5500 3300
Wire Wire Line
	4250 3150 5500 3150
$Comp
L Connector_Generic:Conn_01x03 J1
U 1 1 60E4B160
P 5500 3500
F 0 "J1" V 5372 3680 50  0000 L CNN
F 1 "Conn_01x03" V 5463 3680 50  0000 L CNN
F 2 "Connector_JST:JST_XH_S3B-XH-A_1x03_P2.50mm_Horizontal" H 5500 3500 50  0001 C CNN
F 3 "~" H 5500 3500 50  0001 C CNN
F 4 "0" H 5500 3500 50  0001 C CNN "JLCPCB BOM"
	1    5500 3500
	0    1    1    0   
$EndComp
Wire Wire Line
	5600 3300 5600 3150
Wire Wire Line
	5600 3150 6250 3150
$Comp
L power:GND #PWR02
U 1 1 60E4FA70
P 6250 3150
F 0 "#PWR02" H 6250 2900 50  0001 C CNN
F 1 "GND" V 6255 3022 50  0000 R CNN
F 2 "" H 6250 3150 50  0001 C CNN
F 3 "" H 6250 3150 50  0001 C CNN
	1    6250 3150
	0    -1   -1   0   
$EndComp
$Comp
L power:+5V #PWR0101
U 1 1 60E51AAE
P 7600 2900
F 0 "#PWR0101" H 7600 2750 50  0001 C CNN
F 1 "+5V" V 7615 3028 50  0000 L CNN
F 2 "" H 7600 2900 50  0001 C CNN
F 3 "" H 7600 2900 50  0001 C CNN
	1    7600 2900
	0    1    1    0   
$EndComp
NoConn ~ 6150 3900
NoConn ~ 6000 3900
NoConn ~ 5850 3900
NoConn ~ 5700 3900
Wire Wire Line
	6000 2900 7600 2900
Wire Wire Line
	6700 4500 6850 4500
Text Label 4000 2250 0    50   ~ 0
+5V
Text Label 3200 2250 0    50   ~ 0
+3V
$EndSCHEMATC