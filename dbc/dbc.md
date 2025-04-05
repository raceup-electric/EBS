# DBC Documentation

### can2.dbc

#### Message Index

| Message | ID (dec) | ID (hex) | DLC | Comment |
| ------- | -------- | -------- | --- | ------- |
| [TanksEBS](#tanksebs) | 60 | 0x3c | 2 |  |
| [CarMission](#carmission) | 71 | 0x47 | 1 |  |
| [PcuFault](#pcufault) | 81 | 0x51 | 1 |  |
| [Paddle](#paddle) | 82 | 0x52 | 1 |  |
| [Driver](#driver) | 83 | 0x53 | 4 |  |
| [BmsLv1](#bmslv1) | 84 | 0x54 | 7 |  |
| [BmsLv2](#bmslv2) | 85 | 0x55 | 7 |  |
| [BmsHv1](#bmshv1) | 87 | 0x57 | 7 |  |
| [BmsHv2](#bmshv2) | 88 | 0x58 | 7 |  |
| [Imu1](#imu1) | 96 | 0x60 | 8 |  |
| [Imu2](#imu2) | 97 | 0x61 | 8 |  |
| [Imu3](#imu3) | 98 | 0x62 | 8 |  |
| [IMUCalib](#imucalib) | 99 | 0x63 | 1 | RESERVER FOR IMU mask - DO NOT USE |
| [Map](#map) | 100 | 0x64 | 2 |  |
| [CarStatus](#carstatus) | 101 | 0x65 | 2 |  |
| [CarSettings](#carsettings) | 102 | 0x66 | 8 |  |
| [CheckASB](#checkasb) | 104 | 0x68 | 1 |  |
| [LapStart](#lapstart) | 112 | 0x70 | 1 |  |
| [CarMissionStatus](#carmissionstatus) | 113 | 0x71 | 1 |  |
| [Temp1](#temp1) | 256 | 0x100 | 8 |  |
| [Temp2](#temp2) | 257 | 0x101 | 8 |  |
| [SuspRear](#susprear) | 258 | 0x102 | 3 |  |
| [RESERVED2](#reserved2) | 259 | 0x103 | 0 | RESERVER FOR SMU mask - DO NOT USE |
| [SuspFront](#suspfront) | 260 | 0x104 | 3 |  |
| [TempFrontR](#tempfrontr) | 261 | 0x105 | 3 |  |
| [InvVolt](#invvolt) | 288 | 0x120 | 2 |  |
| [Pcu](#pcu) | 304 | 0x130 | 7 |  |
| [Calib](#calib) | 305 | 0x131 | 1 |  |
| [CalibAck](#caliback) | 306 | 0x132 | 1 |  |
| [PcuSwControl](#pcuswcontrol) | 307 | 0x133 | 1 |  |
| [PcuRfAck](#pcurfack) | 308 | 0x134 | 1 |  |
| [Lem](#lem) | 962 | 0x3c2 | 8 |  |

---

#### TanksEBS

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 60 | 0x3c | 2 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| system_check | 0 | 1 | little_endian | False |  | [0, 1] |  |
| press_left_tank | 1 | 5 | little_endian | False | Bar | [6, 10] |  |
| press_right_tank | 6 | 5 | little_endian | False | Bar | [6, 10] |  |
| sanity_left_sensor | 11 | 1 | little_endian | False |  | [0, 1] |  |
| sanity_right_sensor | 12 | 1 | little_endian | False |  | [0, 1] |  |

---

#### CarMission

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 71 | 0x47 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| Mission | 0 | 3 | little_endian | False |  | [0, 7] |  |

##### Value Maps

| Signal | Key | Value |
| ------ | --- | ----- |
| Mission | 7 | dv_inspection |
| Mission | 6 | dv_ebs_test |
| Mission | 5 | dv_trackdrive |
| Mission | 4 | dv_autocross |
| Mission | 3 | dv_skidpad |
| Mission | 2 | dv_acceleration |
| Mission | 1 | manualy |
| Mission | 0 | none |

---

#### PcuFault

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 81 | 0x51 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| fault_12v | 0 | 1 | little_endian | False | on | [0, 1] |  |
| fault_dv | 1 | 1 | little_endian | False | on | [0, 1] |  |
| fault_24v | 2 | 1 | little_endian | False | on | [0, 1] |  |
| fault_pumpl | 3 | 1 | little_endian | False | on | [0, 1] |  |
| fault_pumpr | 4 | 1 | little_endian | False | on | [0, 1] |  |
| fault_fanbattr | 5 | 1 | little_endian | False | on | [0, 1] |  |
| fault_fanbattl | 6 | 1 | little_endian | False | on | [0, 1] |  |

---

#### Paddle

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 82 | 0x52 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| regen | 0 | 8 | little_endian | False | % | [0, 100] | % of regen paddle travel |

---

#### Driver

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 83 | 0x53 | 4 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| throttle | 0 | 8 | little_endian | False | % | [0, 100] | % of throttle |
| brake | 8 | 8 | little_endian | False | % | [0, 100] | % of brake pedal |
| steering | 16 | 12 | little_endian | True | deg | [-120, 120] | Steering angle in milli radians |
| no_implausibility | 28 | 1 | little_endian | False |  |  |  |
| bre_implausibility | 29 | 1 | little_endian | False |  |  |  |
| pad_implausibility | 30 | 1 | little_endian | False |  |  |  |
| pot_implausibility | 31 | 1 | little_endian | False |  |  |  |

---

#### BmsLv1

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 84 | 0x54 | 7 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| max_volt | 0 | 16 | little_endian | False | mV |  |  |
| min_volt | 16 | 16 | little_endian | False | mV |  |  |
| avg_volt | 32 | 16 | little_endian | False | mV |  |  |
| soc | 48 | 8 | little_endian | False | % | [0, 100] |  |

---

#### BmsLv2

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 85 | 0x55 | 7 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| max_temp | 0 | 16 | little_endian | False | C |  |  |
| min_temp | 16 | 16 | little_endian | False | C |  |  |
| avg_temp | 32 | 16 | little_endian | False | C |  |  |
| fan_speed | 48 | 8 | little_endian | False | % | [0, 100] |  |

---

#### BmsHv1

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 87 | 0x57 | 7 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| max_volt | 0 | 16 | little_endian | False | mV |  | Maximum cell voltage in mv |
| min_volt | 16 | 16 | little_endian | False | mV |  | Minimum cell voltage in mv |
| avg_volt | 32 | 16 | little_endian | False | mV |  | Average cell voltage in mv |
| soc | 48 | 8 | little_endian | False | % | [0, 100] | HV battery SOC in % |

---

#### BmsHv2

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 88 | 0x58 | 7 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| max_temp | 0 | 16 | little_endian | False | C |  | Maximum cell temperature in celsius |
| min_temp | 16 | 16 | little_endian | False | C |  | Minimum cell temperature in celsius |
| avg_temp | 32 | 16 | little_endian | False | C |  | Average cell temperature in celsius |
| fan_speed | 48 | 8 | little_endian | False | % | [0, 100] | % of HV battery fan speed |

---

#### Imu1

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 96 | 0x60 | 8 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| acc_x | 0 | 32 | little_endian | True | m/s^2 |  | Acceleration on x axis in m/s^2 |
| acc_y | 32 | 32 | little_endian | True | m/s^2 |  | Acceleration on y axis in m/s^2 |

---

#### Imu2

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 97 | 0x61 | 8 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| acc_z | 0 | 32 | little_endian | True | m/s^2 |  | Acceleration on z axis in m/s^2 |
| omega_x | 32 | 32 | little_endian | True | rad/s |  | Angular speed on x axis in rad/s |

---

#### Imu3

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 98 | 0x62 | 8 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| omega_y | 0 | 32 | little_endian | True | rad/s |  | Angular speed on x axis in rad/s |
| omega_z | 32 | 32 | little_endian | True | rad/s |  | Angular speed on x axis in rad/s |

---

#### IMUCalib

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 99 | 0x63 | 1 |

**Comment:** RESERVER FOR IMU mask - DO NOT USE

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| start_imu_calibration | 0 | 1 | little_endian | False |  |  |  |

---

#### Map

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 100 | 0x64 | 2 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| power | 0 | 4 | little_endian | False | map | [1, 12] | Map selected number |
| regen | 4 | 4 | little_endian | False | map | [1, 12] | Map selected for regen braking |
| torque_rep | 8 | 4 | little_endian | False | map | [0, 12] | Map selected for torque repeartition |

---

#### CarStatus

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 101 | 0x65 | 2 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| HV | 0 | 1 | little_endian | False | on |  |  |
| R2D | 1 | 1 | little_endian | False | on |  |  |
| RF | 2 | 1 | little_endian | False | enabled |  |  |
| AIR1 | 3 | 1 | little_endian | False | closed |  |  |
| AIR2 | 4 | 1 | little_endian | False | closed |  |  |
| precharge | 5 | 1 | little_endian | False | done |  |  |
| speed | 8 | 8 | little_endian | False | km/h |  |  |

---

#### CarSettings

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 102 | 0x66 | 8 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| max_regen_current | 0 | 8 | little_endian | False | A | [0, 150] | Maximum Regen Current |
| pwr_limit | 8 | 8 | little_endian | False | kW | [0, 80] | Maximum power limit |
| speed_lim | 16 | 8 | little_endian | False | krpm |  | Maximum Speed Limit |
| max_pos_trq | 24 | 8 | little_endian | False | Nm |  | Maximum Positive Torque |
| max_neg_trq | 32 | 8 | little_endian | True | Nm |  | Maximum Negative Torque |
| front_motor_repartition | 40 | 8 | little_endian | False | % |  | Torque Repartition Front |
| rear_motor_repartition | 48 | 8 | little_endian | False | % |  | Torque Repartition Rear |
| torque_vectoring | 56 | 1 | little_endian | False | on | [0, 1] | Torque Vectoring enabled |

---

#### CheckASB

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 104 | 0x68 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| Mode | 0 | 1 | little_endian | False |  | [0, 2] |  |
| response_status | 1 | 3 | little_endian | False |  | [0, 2] |  |

##### Value Maps

| Signal | Key | Value |
| ------ | --- | ----- |
| response_status | 2 | Error |
| response_status | 1 | Failure |
| response_status | 0 | Success |

---

#### LapStart

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 112 | 0x70 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| start | 0 | 8 | little_endian | False | start | [0, 1] |  |

---

#### CarMissionStatus

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 113 | 0x71 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| Mission | 0 | 3 | little_endian | False |  | [0, 7] |  |
| MissionStatus | 3 | 2 | little_endian | False |  | [0, 2] |  |

##### Value Maps

| Signal | Key | Value |
| ------ | --- | ----- |
| MissionStatus | 2 | Mission_finished |
| MissionStatus | 1 | Mission_running |
| MissionStatus | 0 | Mission_not_running |

---

#### Temp1

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 256 | 0x100 | 8 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| temp_motor_post_L | 0 | 16 | little_endian | False | C |  |  |
| temp_motor_pre_L | 16 | 16 | little_endian | False | C |  |  |
| temp_motor_pre_R | 32 | 16 | little_endian | False | C |  |  |
| temp_coldplate_pre_R | 48 | 16 | little_endian | False | C |  |  |

---

#### Temp2

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 257 | 0x101 | 8 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| temp_cold_pre_L | 0 | 16 | little_endian | False | C |  |  |
| temp_cold_post_R | 16 | 16 | little_endian | False | C |  |  |
| temp_cold_post_L | 32 | 16 | little_endian | False | C |  |  |
| temp_mot_post_R | 48 | 16 | little_endian | False | C |  |  |

---

#### SuspRear

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 258 | 0x102 | 3 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| susp_rr | 0 | 12 | little_endian | False | mm |  | RR suspension travel in mm |
| susp_rl | 12 | 12 | little_endian | False | mm |  | RL suspension travel in mm |

---

#### RESERVED2

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 259 | 0x103 | 0 |

**Comment:** RESERVER FOR SMU mask - DO NOT USE

_No signals defined_


---

#### SuspFront

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 260 | 0x104 | 3 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| susp_fr | 0 | 12 | little_endian | False | mm |  | FR suspension travel in mm |
| susp_fl | 12 | 12 | little_endian | False | mm |  | FL suspension travel in mm |

---

#### TempFrontR

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 261 | 0x105 | 3 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| temp_mot_pot_FR | 0 | 10 | little_endian | False | C |  |  |
| temp_mot_pre_FR | 10 | 10 | little_endian | False | C |  |  |

---

#### InvVolt

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 288 | 0x120 | 2 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| car_voltage | 0 | 16 | little_endian | False | V | [0, 600] | Voltage seen from car side (inverter) in volts |

---

#### Pcu

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 304 | 0x130 | 7 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| mode | 0 | 2 | little_endian | False |  | [0, 2] |  |
| rf | 2 | 1 | little_endian | False | on | [0, 1] |  |
| enable_dv | 2 | 1 | little_endian | False | on | [0, 1] |  |
| enable_embedded | 3 | 1 | little_endian | False | on | [0, 1] |  |
| pump_enable_left | 8 | 1 | little_endian | False | on | [0, 1] |  |
| pump_speed_left | 9 | 7 | little_endian | False | % | [0, 100] |  |
| pump_enable_right | 16 | 1 | little_endian | False | on | [0, 1] |  |
| pump_speed_right | 17 | 7 | little_endian | False | % | [0, 100] |  |
| fanrad_enable_left | 24 | 1 | little_endian | False | on | [0, 1] |  |
| fanrad_speed_left | 25 | 7 | little_endian | False | % | [0, 100] |  |
| fanrad_enable_right | 32 | 1 | little_endian | False | on | [0, 1] |  |
| fanrad_speed_right | 33 | 7 | little_endian | False | % | [0, 100] |  |
| fanbatt_enable_left | 40 | 1 | little_endian | False | on | [0, 1] |  |
| fanbatt_speed_left | 41 | 7 | little_endian | False | % | [0, 100] |  |
| fanbatt_enable_right | 48 | 1 | little_endian | False | on | [0, 1] |  |
| fanbatt_speed_right | 49 | 7 | little_endian | False | % | [0, 100] |  |

---

#### Calib

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 305 | 0x131 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| position | 0 | 8 | little_endian | False | high | [0, 1] | Set calibration of rest value (0) or 100% (1) |

---

#### CalibAck

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 306 | 0x132 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| position | 0 | 8 | little_endian | False | high | [0, 1] |  |

---

#### PcuSwControl

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 307 | 0x133 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| pump | 0 | 1 | little_endian | False |  | [0, 1] |  |
| fan | 1 | 1 | little_endian | False |  | [0, 1] |  |

##### Value Maps

| Signal | Key | Value |
| ------ | --- | ----- |
| pump | 1 | On |
| pump | 0 | Off |
| fan | 1 | On |
| fan | 0 | Off |

---

#### PcuRfAck

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 308 | 0x134 | 1 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| rf_signalAck | 0 | 1 | little_endian | False | on | [0, 1] |  |

---

#### Lem

| ID (dec) | ID (hex) | DLC |
| -------- | -------- | --- |
| 962 | 0x3c2 | 8 |

| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |
| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |
| current | 7 | 32 | big_endian | False | mA |  | Current seen from LEM on car side (PDB) |

---

