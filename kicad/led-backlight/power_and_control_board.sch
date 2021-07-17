EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 3 3
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
L Regulator_Linear:AMS1117-3.3 U3
U 1 1 60E65D35
P 6450 3550
F 0 "U3" H 6450 3699 50  0000 C CNN
F 1 "AMS1117-3.3" H 6450 3790 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-223-3_TabPin2" H 6450 3750 50  0001 C CNN
F 3 "http://www.advanced-monolithic.com/pdf/ds1117.pdf" H 6550 3300 50  0001 C CNN
F 4 "C6186" H 6450 3550 50  0001 C CNN "LCSC Part"
	1    6450 3550
	-1   0    0    1   
$EndComp
$Comp
L Device:CP1_Small C5
U 1 1 60E664B4
P 5900 3350
F 0 "C5" H 6100 3300 50  0000 R CNN
F 1 "22uF" H 6300 3400 50  0000 R CNN
F 2 "Capacitor_Tantalum_SMD:CP_EIA-3528-12_Kemet-T" H 5900 3350 50  0001 C CNN
F 3 "~" H 5900 3350 50  0001 C CNN
F 4 "C8020" H 5900 3350 50  0001 C CNN "LCSC Part"
	1    5900 3350
	-1   0    0    1   
$EndComp
$Comp
L Switch:SW_MEC_5E SW1
U 1 1 60E66D50
P 5450 3900
F 0 "SW1" V 5450 4285 50  0000 C CNN
F 1 "SW_MEC_5E" V 5450 3700 50  0000 C CNN
F 2 "Button_Switch_THT:SW_PUSH-12mm" H 5450 4200 50  0001 C CNN
F 3 "http://www.apem.com/int/index.php?controller=attachment&id_attachment=1371" H 5450 4200 50  0001 C CNN
F 4 "0" H 5450 3900 50  0001 C CNN "JLCPCB BOM"
	1    5450 3900
	0    -1   -1   0   
$EndComp
Text HLabel 5200 2950 0    50   Output ~ 0
mode_ref
Text HLabel 5350 4350 3    50   Output ~ 0
mode_btn
Wire Wire Line
	6450 3250 5900 3250
Wire Wire Line
	6950 3250 6450 3250
Connection ~ 6450 3250
Wire Wire Line
	6950 3450 6950 3550
Wire Wire Line
	6950 3550 6750 3550
Wire Wire Line
	6150 3550 5900 3550
Wire Wire Line
	5900 3550 5900 3450
Connection ~ 6950 3550
$Comp
L power:+5V #PWR08
U 1 1 60E6A9F4
P 7550 3550
F 0 "#PWR08" H 7550 3400 50  0001 C CNN
F 1 "+5V" V 7565 3678 50  0000 L CNN
F 2 "" H 7550 3550 50  0001 C CNN
F 3 "" H 7550 3550 50  0001 C CNN
	1    7550 3550
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR07
U 1 1 60E6B19C
P 6450 3250
F 0 "#PWR07" H 6450 3000 50  0001 C CNN
F 1 "GND" H 6455 3077 50  0000 C CNN
F 2 "" H 6450 3250 50  0001 C CNN
F 3 "" H 6450 3250 50  0001 C CNN
	1    6450 3250
	-1   0    0    1   
$EndComp
$Comp
L Device:R_POT_US RV1
U 1 1 60E71E6D
P 5350 2950
F 0 "RV1" H 5282 2996 50  0000 R CNN
F 1 "R_POT_US" H 5282 2905 50  0000 R CNN
F 2 "Connector_JST:JST_XH_S3B-XH-A_1x03_P2.50mm_Horizontal" H 5350 2950 50  0001 C CNN
F 3 "~" H 5350 2950 50  0001 C CNN
F 4 "0" H 5350 2950 50  0001 C CNN "JLCPCB BOM"
	1    5350 2950
	-1   0    0    -1  
$EndComp
Wire Wire Line
	5900 3550 5350 3550
Wire Wire Line
	5350 3550 5350 3100
Connection ~ 5900 3550
Wire Wire Line
	5350 3700 5450 3700
Connection ~ 5350 3700
Wire Wire Line
	5350 3700 5350 4100
Wire Wire Line
	5350 4100 5450 4100
Connection ~ 5350 4100
Wire Wire Line
	5350 4100 5350 4350
$Comp
L power:GND #PWR06
U 1 1 60E7C402
P 5350 2800
F 0 "#PWR06" H 5350 2550 50  0001 C CNN
F 1 "GND" H 5355 2627 50  0000 C CNN
F 2 "" H 5350 2800 50  0001 C CNN
F 3 "" H 5350 2800 50  0001 C CNN
	1    5350 2800
	-1   0    0    1   
$EndComp
$Comp
L Device:C_Small C6
U 1 1 60E66209
P 6950 3350
F 0 "C6" H 7042 3396 50  0000 L CNN
F 1 "10uF" H 7042 3305 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric" H 6950 3350 50  0001 C CNN
F 3 "~" H 6950 3350 50  0001 C CNN
F 4 "C15850" H 6950 3350 50  0001 C CNN "LCSC Part"
	1    6950 3350
	1    0    0    -1  
$EndComp
Wire Wire Line
	6950 3550 7550 3550
Wire Wire Line
	5350 3700 5350 3550
Connection ~ 5350 3550
$EndSCHEMATC
