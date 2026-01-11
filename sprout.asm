
target/x86_64-unknown-none/debug/sprout:     file format elf64-x86-64


Disassembly of section .text:

0000000000200000 <main>:
  200000:	48 83 ec 18          	sub    $0x18,%rsp
  200004:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
  200009:	48 c7 c6 00 07 20 00 	mov    $0x200700,%rsi
  200010:	48 89 e7             	mov    %rsp,%rdi
  200013:	ba 12 00 00 00       	mov    $0x12,%edx
  200018:	e8 53 01 00 00       	call   200170 <_ZN4stem7syscall13spawn_process17h499fa940761546ebE>
  20001d:	bf e8 03 00 00       	mov    $0x3e8,%edi
  200022:	e8 99 02 00 00       	call   2002c0 <_ZN4stem7syscall8sleep_ms17h6fed257ec89c0d05E>
  200027:	89 c7                	mov    %eax,%edi
  200029:	e8 02 00 00 00       	call   200030 <_ZN4core6result19Result$LT$T$C$E$GT$2ok17hea2e26444f448d72E>
  20002e:	eb ed                	jmp    20001d <main+0x1d>

0000000000200030 <_ZN4core6result19Result$LT$T$C$E$GT$2ok17hea2e26444f448d72E>:
  200030:	50                   	push   %rax
  200031:	89 3c 24             	mov    %edi,(%rsp)
  200034:	b8 01 00 00 00       	mov    $0x1,%eax
  200039:	31 c9                	xor    %ecx,%ecx
  20003b:	83 3c 24 27          	cmpl   $0x27,(%rsp)
  20003f:	48 0f 44 c1          	cmove  %rcx,%rax
  200043:	48 a9 01 00 00 00    	test   $0x1,%rax
  200049:	74 07                	je     200052 <_ZN4core6result19Result$LT$T$C$E$GT$2ok17hea2e26444f448d72E+0x22>
  20004b:	c6 44 24 06 00       	movb   $0x0,0x6(%rsp)
  200050:	eb 05                	jmp    200057 <_ZN4core6result19Result$LT$T$C$E$GT$2ok17hea2e26444f448d72E+0x27>
  200052:	c6 44 24 06 01       	movb   $0x1,0x6(%rsp)
  200057:	b8 01 00 00 00       	mov    $0x1,%eax
  20005c:	31 c9                	xor    %ecx,%ecx
  20005e:	83 3c 24 27          	cmpl   $0x27,(%rsp)
  200062:	48 0f 44 c1          	cmove  %rcx,%rax
  200066:	48 a9 01 00 00 00    	test   $0x1,%rax
  20006c:	74 02                	je     200070 <_ZN4core6result19Result$LT$T$C$E$GT$2ok17hea2e26444f448d72E+0x40>
  20006e:	eb 00                	jmp    200070 <_ZN4core6result19Result$LT$T$C$E$GT$2ok17hea2e26444f448d72E+0x40>
  200070:	8a 44 24 06          	mov    0x6(%rsp),%al
  200074:	24 01                	and    $0x1,%al
  200076:	59                   	pop    %rcx
  200077:	c3                   	ret
  200078:	cc                   	int3
  200079:	cc                   	int3
  20007a:	cc                   	int3
  20007b:	cc                   	int3
  20007c:	cc                   	int3
  20007d:	cc                   	int3
  20007e:	cc                   	int3
  20007f:	cc                   	int3

0000000000200080 <_ZN4stem2rt10entry_impl17h8cbf0cba2d807151E>:
  200080:	48 83 ec 18          	sub    $0x18,%rsp
  200084:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  200089:	e8 72 ff ff ff       	call   200000 <main>
  20008e:	89 c7                	mov    %eax,%edi
  200090:	89 7c 24 14          	mov    %edi,0x14(%rsp)
  200094:	e8 b7 01 00 00       	call   200250 <_ZN4stem7syscall4exit17h7ea65e09d4c25299E>
  200099:	cc                   	int3
  20009a:	cc                   	int3
  20009b:	cc                   	int3
  20009c:	cc                   	int3
  20009d:	cc                   	int3
  20009e:	cc                   	int3
  20009f:	cc                   	int3

00000000002000a0 <_start>:
  2000a0:	50                   	push   %rax
  2000a1:	48 89 3c 24          	mov    %rdi,(%rsp)
  2000a5:	e8 d6 ff ff ff       	call   200080 <_ZN4stem2rt10entry_impl17h8cbf0cba2d807151E>
  2000aa:	cc                   	int3
  2000ab:	cc                   	int3
  2000ac:	cc                   	int3
  2000ad:	cc                   	int3
  2000ae:	cc                   	int3
  2000af:	cc                   	int3

00000000002000b0 <_ZN4stem7syscall11debug_write17h3931624dbdbc70d6E>:
  2000b0:	48 81 ec 88 00 00 00 	sub    $0x88,%rsp
  2000b7:	48 89 14 24          	mov    %rdx,(%rsp)
  2000bb:	48 89 f0             	mov    %rsi,%rax
  2000be:	48 8b 34 24          	mov    (%rsp),%rsi
  2000c2:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  2000c7:	48 89 f8             	mov    %rdi,%rax
  2000ca:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
  2000cf:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  2000d4:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
  2000d9:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
  2000de:	48 89 74 24 28       	mov    %rsi,0x28(%rsp)
  2000e3:	48 89 7c 24 38       	mov    %rdi,0x38(%rsp)
  2000e8:	48 89 74 24 40       	mov    %rsi,0x40(%rsp)
  2000ed:	c7 44 24 54 02 00 00 	movl   $0x2,0x54(%rsp)
  2000f4:	00 
  2000f5:	48 89 7c 24 58       	mov    %rdi,0x58(%rsp)
  2000fa:	48 89 74 24 60       	mov    %rsi,0x60(%rsp)
  2000ff:	48 c7 44 24 68 00 00 	movq   $0x0,0x68(%rsp)
  200106:	00 00 
  200108:	48 c7 44 24 70 00 00 	movq   $0x0,0x70(%rsp)
  20010f:	00 00 
  200111:	48 c7 44 24 78 00 00 	movq   $0x0,0x78(%rsp)
  200118:	00 00 
  20011a:	48 c7 84 24 80 00 00 	movq   $0x0,0x80(%rsp)
  200121:	00 00 00 00 00 
  200126:	31 c0                	xor    %eax,%eax
  200128:	41 89 c1             	mov    %eax,%r9d
  20012b:	b8 02 00 00 00       	mov    $0x2,%eax
  200130:	4c 89 ca             	mov    %r9,%rdx
  200133:	4d 89 ca             	mov    %r9,%r10
  200136:	4d 89 c8             	mov    %r9,%r8
  200139:	0f 05                	syscall
  20013b:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
  200140:	48 89 44 24 48       	mov    %rax,0x48(%rsp)
  200145:	48 8b 74 24 48       	mov    0x48(%rsp),%rsi
  20014a:	48 89 74 24 30       	mov    %rsi,0x30(%rsp)
  20014f:	e8 cc 03 00 00       	call   200520 <_ZN3abi6errors5errno17he7228877f6a46568E>
  200154:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  200159:	48 81 c4 88 00 00 00 	add    $0x88,%rsp
  200160:	c3                   	ret
  200161:	cc                   	int3
  200162:	cc                   	int3
  200163:	cc                   	int3
  200164:	cc                   	int3
  200165:	cc                   	int3
  200166:	cc                   	int3
  200167:	cc                   	int3
  200168:	cc                   	int3
  200169:	cc                   	int3
  20016a:	cc                   	int3
  20016b:	cc                   	int3
  20016c:	cc                   	int3
  20016d:	cc                   	int3
  20016e:	cc                   	int3
  20016f:	cc                   	int3

0000000000200170 <_ZN4stem7syscall13spawn_process17h499fa940761546ebE>:
  200170:	48 81 ec 98 00 00 00 	sub    $0x98,%rsp
  200177:	48 89 14 24          	mov    %rdx,(%rsp)
  20017b:	48 89 f0             	mov    %rsi,%rax
  20017e:	48 8b 34 24          	mov    (%rsp),%rsi
  200182:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  200187:	48 89 f8             	mov    %rdi,%rax
  20018a:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
  20018f:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  200194:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
  200199:	48 89 7c 24 30       	mov    %rdi,0x30(%rsp)
  20019e:	48 89 74 24 38       	mov    %rsi,0x38(%rsp)
  2001a3:	48 89 7c 24 48       	mov    %rdi,0x48(%rsp)
  2001a8:	48 89 74 24 50       	mov    %rsi,0x50(%rsp)
  2001ad:	e8 5e 02 00 00       	call   200410 <_ZN4core3str21_$LT$impl$u20$str$GT$3len17heef7e59f49012ad5E>
  2001b2:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
  2001b7:	48 89 c6             	mov    %rax,%rsi
  2001ba:	c7 44 24 64 07 00 00 	movl   $0x7,0x64(%rsp)
  2001c1:	00 
  2001c2:	48 89 7c 24 68       	mov    %rdi,0x68(%rsp)
  2001c7:	48 89 74 24 70       	mov    %rsi,0x70(%rsp)
  2001cc:	48 c7 44 24 78 00 00 	movq   $0x0,0x78(%rsp)
  2001d3:	00 00 
  2001d5:	48 c7 84 24 80 00 00 	movq   $0x0,0x80(%rsp)
  2001dc:	00 00 00 00 00 
  2001e1:	48 c7 84 24 88 00 00 	movq   $0x0,0x88(%rsp)
  2001e8:	00 00 00 00 00 
  2001ed:	48 c7 84 24 90 00 00 	movq   $0x0,0x90(%rsp)
  2001f4:	00 00 00 00 00 
  2001f9:	31 c0                	xor    %eax,%eax
  2001fb:	41 89 c1             	mov    %eax,%r9d
  2001fe:	b8 07 00 00 00       	mov    $0x7,%eax
  200203:	4c 89 ca             	mov    %r9,%rdx
  200206:	4d 89 ca             	mov    %r9,%r10
  200209:	4d 89 c8             	mov    %r9,%r8
  20020c:	0f 05                	syscall
  20020e:	48 89 44 24 58       	mov    %rax,0x58(%rsp)
  200213:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
  200218:	48 89 74 24 40       	mov    %rsi,0x40(%rsp)
  20021d:	48 8d 7c 24 20       	lea    0x20(%rsp),%rdi
  200222:	e8 f9 02 00 00       	call   200520 <_ZN3abi6errors5errno17he7228877f6a46568E>
  200227:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
  20022c:	48 8d 74 24 20       	lea    0x20(%rsp),%rsi
  200231:	e8 6a 01 00 00       	call   2003a0 <_ZN4core6result19Result$LT$T$C$E$GT$3map17hbf90f25640198eeeE>
  200236:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  20023b:	48 81 c4 98 00 00 00 	add    $0x98,%rsp
  200242:	c3                   	ret
  200243:	cc                   	int3
  200244:	cc                   	int3
  200245:	cc                   	int3
  200246:	cc                   	int3
  200247:	cc                   	int3
  200248:	cc                   	int3
  200249:	cc                   	int3
  20024a:	cc                   	int3
  20024b:	cc                   	int3
  20024c:	cc                   	int3
  20024d:	cc                   	int3
  20024e:	cc                   	int3
  20024f:	cc                   	int3

0000000000200250 <_ZN4stem7syscall4exit17h7ea65e09d4c25299E>:
  200250:	48 83 ec 48          	sub    $0x48,%rsp
  200254:	89 7c 24 04          	mov    %edi,0x4(%rsp)
  200258:	48 63 ff             	movslq %edi,%rdi
  20025b:	c7 44 24 14 01 00 00 	movl   $0x1,0x14(%rsp)
  200262:	00 
  200263:	48 89 7c 24 18       	mov    %rdi,0x18(%rsp)
  200268:	48 c7 44 24 20 00 00 	movq   $0x0,0x20(%rsp)
  20026f:	00 00 
  200271:	48 c7 44 24 28 00 00 	movq   $0x0,0x28(%rsp)
  200278:	00 00 
  20027a:	48 c7 44 24 30 00 00 	movq   $0x0,0x30(%rsp)
  200281:	00 00 
  200283:	48 c7 44 24 38 00 00 	movq   $0x0,0x38(%rsp)
  20028a:	00 00 
  20028c:	48 c7 44 24 40 00 00 	movq   $0x0,0x40(%rsp)
  200293:	00 00 
  200295:	31 c0                	xor    %eax,%eax
  200297:	41 89 c1             	mov    %eax,%r9d
  20029a:	b8 01 00 00 00       	mov    $0x1,%eax
  20029f:	4c 89 ce             	mov    %r9,%rsi
  2002a2:	4c 89 ca             	mov    %r9,%rdx
  2002a5:	4d 89 ca             	mov    %r9,%r10
  2002a8:	4d 89 c8             	mov    %r9,%r8
  2002ab:	0f 05                	syscall
  2002ad:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  2002b2:	48 c7 c7 40 07 20 00 	mov    $0x200740,%rdi
  2002b9:	e8 d2 01 00 00       	call   200490 <_ZN4core4hint21unreachable_unchecked17h7092067e6d7aad74E>
  2002be:	cc                   	int3
  2002bf:	cc                   	int3

00000000002002c0 <_ZN4stem7syscall8sleep_ms17h6fed257ec89c0d05E>:
  2002c0:	48 83 ec 68          	sub    $0x68,%rsp
  2002c4:	48 89 7c 24 18       	mov    %rdi,0x18(%rsp)
  2002c9:	c7 44 24 34 03 00 00 	movl   $0x3,0x34(%rsp)
  2002d0:	00 
  2002d1:	48 89 7c 24 38       	mov    %rdi,0x38(%rsp)
  2002d6:	48 c7 44 24 40 00 00 	movq   $0x0,0x40(%rsp)
  2002dd:	00 00 
  2002df:	48 c7 44 24 48 00 00 	movq   $0x0,0x48(%rsp)
  2002e6:	00 00 
  2002e8:	48 c7 44 24 50 00 00 	movq   $0x0,0x50(%rsp)
  2002ef:	00 00 
  2002f1:	48 c7 44 24 58 00 00 	movq   $0x0,0x58(%rsp)
  2002f8:	00 00 
  2002fa:	48 c7 44 24 60 00 00 	movq   $0x0,0x60(%rsp)
  200301:	00 00 
  200303:	31 c0                	xor    %eax,%eax
  200305:	41 89 c1             	mov    %eax,%r9d
  200308:	b8 03 00 00 00       	mov    $0x3,%eax
  20030d:	4c 89 ce             	mov    %r9,%rsi
  200310:	4c 89 ca             	mov    %r9,%rdx
  200313:	4d 89 ca             	mov    %r9,%r10
  200316:	4d 89 c8             	mov    %r9,%r8
  200319:	0f 05                	syscall
  20031b:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
  200320:	48 8b 74 24 28       	mov    0x28(%rsp),%rsi
  200325:	48 89 74 24 20       	mov    %rsi,0x20(%rsp)
  20032a:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
  20032f:	e8 ec 01 00 00       	call   200520 <_ZN3abi6errors5errno17he7228877f6a46568E>
  200334:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
  200339:	e8 12 00 00 00       	call   200350 <_ZN4core6result19Result$LT$T$C$E$GT$3map17h34500ec096748500E>
  20033e:	48 83 c4 68          	add    $0x68,%rsp
  200342:	c3                   	ret
  200343:	cc                   	int3
  200344:	cc                   	int3
  200345:	cc                   	int3
  200346:	cc                   	int3
  200347:	cc                   	int3
  200348:	cc                   	int3
  200349:	cc                   	int3
  20034a:	cc                   	int3
  20034b:	cc                   	int3
  20034c:	cc                   	int3
  20034d:	cc                   	int3
  20034e:	cc                   	int3
  20034f:	cc                   	int3

0000000000200350 <_ZN4core6result19Result$LT$T$C$E$GT$3map17h34500ec096748500E>:
  200350:	48 83 ec 28          	sub    $0x28,%rsp
  200354:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  200359:	8b 07                	mov    (%rdi),%eax
  20035b:	48 a9 01 00 00 00    	test   $0x1,%rax
  200361:	74 12                	je     200375 <_ZN4core6result19Result$LT$T$C$E$GT$3map17h34500ec096748500E+0x25>
  200363:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  200368:	8b 40 04             	mov    0x4(%rax),%eax
  20036b:	89 44 24 24          	mov    %eax,0x24(%rsp)
  20036f:	89 44 24 10          	mov    %eax,0x10(%rsp)
  200373:	eb 1b                	jmp    200390 <_ZN4core6result19Result$LT$T$C$E$GT$3map17h34500ec096748500E+0x40>
  200375:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  20037a:	48 8b 78 08          	mov    0x8(%rax),%rdi
  20037e:	48 89 7c 24 18       	mov    %rdi,0x18(%rsp)
  200383:	e8 f8 00 00 00       	call   200480 <_ZN4stem7syscall8sleep_ms28_$u7b$$u7b$closure$u7d$$u7d$17hdf38482ae94a9a9fE>
  200388:	c7 44 24 10 27 00 00 	movl   $0x27,0x10(%rsp)
  20038f:	00 
  200390:	8b 44 24 10          	mov    0x10(%rsp),%eax
  200394:	48 83 c4 28          	add    $0x28,%rsp
  200398:	c3                   	ret
  200399:	cc                   	int3
  20039a:	cc                   	int3
  20039b:	cc                   	int3
  20039c:	cc                   	int3
  20039d:	cc                   	int3
  20039e:	cc                   	int3
  20039f:	cc                   	int3

00000000002003a0 <_ZN4core6result19Result$LT$T$C$E$GT$3map17hbf90f25640198eeeE>:
  2003a0:	48 83 ec 38          	sub    $0x38,%rsp
  2003a4:	48 89 74 24 08       	mov    %rsi,0x8(%rsp)
  2003a9:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
  2003ae:	48 89 7c 24 18       	mov    %rdi,0x18(%rsp)
  2003b3:	8b 06                	mov    (%rsi),%eax
  2003b5:	48 a9 01 00 00 00    	test   $0x1,%rax
  2003bb:	74 1c                	je     2003d9 <_ZN4core6result19Result$LT$T$C$E$GT$3map17hbf90f25640198eeeE+0x39>
  2003bd:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  2003c2:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
  2003c7:	8b 49 04             	mov    0x4(%rcx),%ecx
  2003ca:	89 4c 24 34          	mov    %ecx,0x34(%rsp)
  2003ce:	89 48 04             	mov    %ecx,0x4(%rax)
  2003d1:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
  2003d7:	eb 25                	jmp    2003fe <_ZN4core6result19Result$LT$T$C$E$GT$3map17hbf90f25640198eeeE+0x5e>
  2003d9:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  2003de:	48 8b 78 08          	mov    0x8(%rax),%rdi
  2003e2:	48 89 7c 24 28       	mov    %rdi,0x28(%rsp)
  2003e7:	e8 74 00 00 00       	call   200460 <_ZN4stem7syscall13spawn_process28_$u7b$$u7b$closure$u7d$$u7d$17h2336e7c6e65b4d8eE>
  2003ec:	48 89 c1             	mov    %rax,%rcx
  2003ef:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  2003f4:	48 89 48 08          	mov    %rcx,0x8(%rax)
  2003f8:	c7 00 00 00 00 00    	movl   $0x0,(%rax)
  2003fe:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  200403:	48 83 c4 38          	add    $0x38,%rsp
  200407:	c3                   	ret
  200408:	cc                   	int3
  200409:	cc                   	int3
  20040a:	cc                   	int3
  20040b:	cc                   	int3
  20040c:	cc                   	int3
  20040d:	cc                   	int3
  20040e:	cc                   	int3
  20040f:	cc                   	int3

0000000000200410 <_ZN4core3str21_$LT$impl$u20$str$GT$3len17heef7e59f49012ad5E>:
  200410:	48 83 ec 10          	sub    $0x10,%rsp
  200414:	48 89 f0             	mov    %rsi,%rax
  200417:	48 89 3c 24          	mov    %rdi,(%rsp)
  20041b:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  200420:	48 83 c4 10          	add    $0x10,%rsp
  200424:	c3                   	ret
  200425:	cc                   	int3
  200426:	cc                   	int3
  200427:	cc                   	int3
  200428:	cc                   	int3
  200429:	cc                   	int3
  20042a:	cc                   	int3
  20042b:	cc                   	int3
  20042c:	cc                   	int3
  20042d:	cc                   	int3
  20042e:	cc                   	int3
  20042f:	cc                   	int3

0000000000200430 <_RNvCskdKJRKLKjqM_7___rustc17rust_begin_unwind>:
  200430:	48 83 ec 18          	sub    $0x18,%rsp
  200434:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
  200439:	48 c7 c6 58 07 20 00 	mov    $0x200758,%rsi
  200440:	48 89 e7             	mov    %rsp,%rdi
  200443:	ba 23 00 00 00       	mov    $0x23,%edx
  200448:	e8 63 fc ff ff       	call   2000b0 <_ZN4stem7syscall11debug_write17h3931624dbdbc70d6E>
  20044d:	bf 65 00 00 00       	mov    $0x65,%edi
  200452:	e8 f9 fd ff ff       	call   200250 <_ZN4stem7syscall4exit17h7ea65e09d4c25299E>
  200457:	cc                   	int3
  200458:	cc                   	int3
  200459:	cc                   	int3
  20045a:	cc                   	int3
  20045b:	cc                   	int3
  20045c:	cc                   	int3
  20045d:	cc                   	int3
  20045e:	cc                   	int3
  20045f:	cc                   	int3

0000000000200460 <_ZN4stem7syscall13spawn_process28_$u7b$$u7b$closure$u7d$$u7d$17h2336e7c6e65b4d8eE>:
  200460:	48 83 ec 10          	sub    $0x10,%rsp
  200464:	48 89 f8             	mov    %rdi,%rax
  200467:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
  20046c:	48 83 c4 10          	add    $0x10,%rsp
  200470:	c3                   	ret
  200471:	cc                   	int3
  200472:	cc                   	int3
  200473:	cc                   	int3
  200474:	cc                   	int3
  200475:	cc                   	int3
  200476:	cc                   	int3
  200477:	cc                   	int3
  200478:	cc                   	int3
  200479:	cc                   	int3
  20047a:	cc                   	int3
  20047b:	cc                   	int3
  20047c:	cc                   	int3
  20047d:	cc                   	int3
  20047e:	cc                   	int3
  20047f:	cc                   	int3

0000000000200480 <_ZN4stem7syscall8sleep_ms28_$u7b$$u7b$closure$u7d$$u7d$17hdf38482ae94a9a9fE>:
  200480:	48 83 ec 10          	sub    $0x10,%rsp
  200484:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
  200489:	48 83 c4 10          	add    $0x10,%rsp
  20048d:	c3                   	ret
  20048e:	cc                   	int3
  20048f:	cc                   	int3

0000000000200490 <_ZN4core4hint21unreachable_unchecked17h7092067e6d7aad74E>:
  200490:	50                   	push   %rax
  200491:	e8 0a 00 00 00       	call   2004a0 <_ZN4core4hint21unreachable_unchecked18precondition_check17hb20bdec80e4b0292E>
  200496:	0f 0b                	ud2
  200498:	cc                   	int3
  200499:	cc                   	int3
  20049a:	cc                   	int3
  20049b:	cc                   	int3
  20049c:	cc                   	int3
  20049d:	cc                   	int3
  20049e:	cc                   	int3
  20049f:	cc                   	int3

00000000002004a0 <_ZN4core4hint21unreachable_unchecked18precondition_check17hb20bdec80e4b0292E>:
  2004a0:	48 83 ec 58          	sub    $0x58,%rsp
  2004a4:	48 89 fa             	mov    %rdi,%rdx
  2004a7:	48 c7 44 24 40 7b 07 	movq   $0x20077b,0x40(%rsp)
  2004ae:	20 00 
  2004b0:	48 c7 44 24 48 c7 00 	movq   $0xc7,0x48(%rsp)
  2004b7:	00 00 
  2004b9:	48 c7 44 24 30 7b 07 	movq   $0x20077b,0x30(%rsp)
  2004c0:	20 00 
  2004c2:	48 c7 44 24 38 c7 00 	movq   $0xc7,0x38(%rsp)
  2004c9:	00 00 
  2004cb:	48 8d 44 24 30       	lea    0x30(%rsp),%rax
  2004d0:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
  2004d5:	48 89 04 24          	mov    %rax,(%rsp)
  2004d9:	48 c7 44 24 08 01 00 	movq   $0x1,0x8(%rsp)
  2004e0:	00 00 
  2004e2:	48 8b 0d 5f 03 00 00 	mov    0x35f(%rip),%rcx        # 200848 <_ZN4core9panicking9panic_fmt17h6fa4551a01b5a063E+0x168>
  2004e9:	48 8b 05 60 03 00 00 	mov    0x360(%rip),%rax        # 200850 <_ZN4core9panicking9panic_fmt17h6fa4551a01b5a063E+0x170>
  2004f0:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
  2004f5:	48 89 44 24 28       	mov    %rax,0x28(%rsp)
  2004fa:	b8 08 00 00 00       	mov    $0x8,%eax
  2004ff:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  200504:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
  20050b:	00 00 
  20050d:	31 c0                	xor    %eax,%eax
  20050f:	48 89 e7             	mov    %rsp,%rdi
  200512:	31 f6                	xor    %esi,%esi
  200514:	e8 67 01 00 00       	call   200680 <_ZN4core9panicking18panic_nounwind_fmt17h04b61d842c888656E>
  200519:	cc                   	int3
  20051a:	cc                   	int3
  20051b:	cc                   	int3
  20051c:	cc                   	int3
  20051d:	cc                   	int3
  20051e:	cc                   	int3
  20051f:	cc                   	int3

0000000000200520 <_ZN3abi6errors5errno17he7228877f6a46568E>:
  200520:	48 83 ec 38          	sub    $0x38,%rsp
  200524:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
  200529:	48 89 7c 24 18       	mov    %rdi,0x18(%rsp)
  20052e:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
  200533:	48 89 74 24 28       	mov    %rsi,0x28(%rsp)
  200538:	48 83 fe 00          	cmp    $0x0,%rsi
  20053c:	7c 16                	jl     200554 <_ZN3abi6errors5errno17he7228877f6a46568E+0x34>
  20053e:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  200543:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
  200548:	48 89 48 08          	mov    %rcx,0x8(%rax)
  20054c:	c7 00 00 00 00 00    	movl   $0x0,(%rax)
  200552:	eb 16                	jmp    20056a <_ZN3abi6errors5errno17he7228877f6a46568E+0x4a>
  200554:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  200559:	48 b9 00 00 00 00 00 	movabs $0x8000000000000000,%rcx
  200560:	00 00 80 
  200563:	48 39 c8             	cmp    %rcx,%rax
  200566:	74 3c                	je     2005a4 <_ZN3abi6errors5errno17he7228877f6a46568E+0x84>
  200568:	eb 0a                	jmp    200574 <_ZN3abi6errors5errno17he7228877f6a46568E+0x54>
  20056a:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  20056f:	48 83 c4 38          	add    $0x38,%rsp
  200573:	c3                   	ret
  200574:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  200579:	48 89 c1             	mov    %rax,%rcx
  20057c:	48 f7 d9             	neg    %rcx
  20057f:	48 89 4c 24 30       	mov    %rcx,0x30(%rsp)
  200584:	48 89 c1             	mov    %rax,%rcx
  200587:	48 f7 d1             	not    %rcx
  20058a:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
  20058f:	48 83 e8 da          	sub    $0xffffffffffffffda,%rax
  200593:	72 1b                	jb     2005b0 <_ZN3abi6errors5errno17he7228877f6a46568E+0x90>
  200595:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  20059a:	48 8b 04 c5 58 08 20 	mov    0x200858(,%rax,8),%rax
  2005a1:	00 
  2005a2:	ff e0                	jmp    *%rax
  2005a4:	48 c7 c7 88 09 20 00 	mov    $0x200988,%rdi
  2005ab:	e8 90 00 00 00       	call   200640 <_ZN4core9panicking11panic_const24panic_const_neg_overflow17he92b6cddf6baa067E>
  2005b0:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  2005b5:	c7 40 04 16 00 00 00 	movl   $0x16,0x4(%rax)
  2005bc:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
  2005c2:	eb a6                	jmp    20056a <_ZN3abi6errors5errno17he7228877f6a46568E+0x4a>
  2005c4:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  2005c9:	c7 40 04 01 00 00 00 	movl   $0x1,0x4(%rax)
  2005d0:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
  2005d6:	eb 92                	jmp    20056a <_ZN3abi6errors5errno17he7228877f6a46568E+0x4a>
  2005d8:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  2005dd:	c7 40 04 0c 00 00 00 	movl   $0xc,0x4(%rax)
  2005e4:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
  2005ea:	e9 7b ff ff ff       	jmp    20056a <_ZN3abi6errors5errno17he7228877f6a46568E+0x4a>
  2005ef:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  2005f4:	c7 40 04 0e 00 00 00 	movl   $0xe,0x4(%rax)
  2005fb:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
  200601:	e9 64 ff ff ff       	jmp    20056a <_ZN3abi6errors5errno17he7228877f6a46568E+0x4a>
  200606:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  20060b:	c7 40 04 16 00 00 00 	movl   $0x16,0x4(%rax)
  200612:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
  200618:	e9 4d ff ff ff       	jmp    20056a <_ZN3abi6errors5errno17he7228877f6a46568E+0x4a>
  20061d:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  200622:	c7 40 04 26 00 00 00 	movl   $0x26,0x4(%rax)
  200629:	c7 00 01 00 00 00    	movl   $0x1,(%rax)
  20062f:	e9 36 ff ff ff       	jmp    20056a <_ZN3abi6errors5errno17he7228877f6a46568E+0x4a>
  200634:	cc                   	int3
  200635:	cc                   	int3
  200636:	cc                   	int3
  200637:	cc                   	int3
  200638:	cc                   	int3
  200639:	cc                   	int3
  20063a:	cc                   	int3
  20063b:	cc                   	int3
  20063c:	cc                   	int3
  20063d:	cc                   	int3
  20063e:	cc                   	int3
  20063f:	cc                   	int3

0000000000200640 <_ZN4core9panicking11panic_const24panic_const_neg_overflow17he92b6cddf6baa067E>:
  200640:	55                   	push   %rbp
  200641:	48 89 e5             	mov    %rsp,%rbp
  200644:	48 83 ec 30          	sub    $0x30,%rsp
  200648:	48 89 fe             	mov    %rdi,%rsi
  20064b:	48 8d 05 ae 03 00 00 	lea    0x3ae(%rip),%rax        # 200a00 <_ZN4core9panicking9panic_fmt17h6fa4551a01b5a063E+0x320>
  200652:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
  200656:	48 c7 45 d8 01 00 00 	movq   $0x1,-0x28(%rbp)
  20065d:	00 
  20065e:	48 c7 45 f0 00 00 00 	movq   $0x0,-0x10(%rbp)
  200665:	00 
  200666:	48 c7 45 e0 08 00 00 	movq   $0x8,-0x20(%rbp)
  20066d:	00 
  20066e:	48 c7 45 e8 00 00 00 	movq   $0x0,-0x18(%rbp)
  200675:	00 
  200676:	48 8d 7d d0          	lea    -0x30(%rbp),%rdi
  20067a:	ff 15 78 03 00 00    	call   *0x378(%rip)        # 2009f8 <_ZN4core9panicking9panic_fmt17h6fa4551a01b5a063E+0x318>

0000000000200680 <_ZN4core9panicking18panic_nounwind_fmt17h04b61d842c888656E>:
  200680:	55                   	push   %rbp
  200681:	48 89 e5             	mov    %rsp,%rbp
  200684:	48 83 ec 50          	sub    $0x50,%rsp
  200688:	48 8b 47 28          	mov    0x28(%rdi),%rax
  20068c:	48 89 45 e0          	mov    %rax,-0x20(%rbp)
  200690:	48 8b 47 20          	mov    0x20(%rdi),%rax
  200694:	48 89 45 d8          	mov    %rax,-0x28(%rbp)
  200698:	48 8b 47 18          	mov    0x18(%rdi),%rax
  20069c:	48 89 45 d0          	mov    %rax,-0x30(%rbp)
  2006a0:	48 8b 47 10          	mov    0x10(%rdi),%rax
  2006a4:	48 89 45 c8          	mov    %rax,-0x38(%rbp)
  2006a8:	48 8b 07             	mov    (%rdi),%rax
  2006ab:	48 8b 4f 08          	mov    0x8(%rdi),%rcx
  2006af:	48 89 4d c0          	mov    %rcx,-0x40(%rbp)
  2006b3:	48 89 45 b8          	mov    %rax,-0x48(%rbp)
  2006b7:	48 8d 45 b8          	lea    -0x48(%rbp),%rax
  2006bb:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
  2006bf:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
  2006c3:	c6 45 f8 00          	movb   $0x0,-0x8(%rbp)
  2006c7:	40 88 75 f9          	mov    %sil,-0x7(%rbp)
  2006cb:	48 8d 7d e8          	lea    -0x18(%rbp),%rdi
  2006cf:	ff 15 1b 03 00 00    	call   *0x31b(%rip)        # 2009f0 <_ZN4core9panicking9panic_fmt17h6fa4551a01b5a063E+0x310>
  2006d5:	cc                   	int3
  2006d6:	cc                   	int3
  2006d7:	cc                   	int3
  2006d8:	cc                   	int3
  2006d9:	cc                   	int3
  2006da:	cc                   	int3
  2006db:	cc                   	int3
  2006dc:	cc                   	int3
  2006dd:	cc                   	int3
  2006de:	cc                   	int3
  2006df:	cc                   	int3

00000000002006e0 <_ZN4core9panicking9panic_fmt17h6fa4551a01b5a063E>:
  2006e0:	55                   	push   %rbp
  2006e1:	48 89 e5             	mov    %rsp,%rbp
  2006e4:	48 83 ec 20          	sub    $0x20,%rsp
  2006e8:	48 89 7d e8          	mov    %rdi,-0x18(%rbp)
  2006ec:	48 89 75 f0          	mov    %rsi,-0x10(%rbp)
  2006f0:	66 c7 45 f8 01 00    	movw   $0x1,-0x8(%rbp)
  2006f6:	48 8d 7d e8          	lea    -0x18(%rbp),%rdi
  2006fa:	ff 15 f0 02 00 00    	call   *0x2f0(%rip)        # 2009f0 <_ZN4core9panicking9panic_fmt17h6fa4551a01b5a063E+0x310>
