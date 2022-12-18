	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"rust1_66.ff00b5fc-cgu.0"
	.def	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h38be00a484bb4f80E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h38be00a484bb4f80E
	.p2align	4, 0x90
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h38be00a484bb4f80E:
.seh_proc _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h38be00a484bb4f80E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	*%rcx
	#APP
	#NO_APP
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start17h50d31464143d65edE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17h50d31464143d65edE
	.globl	_ZN3std2rt10lang_start17h50d31464143d65edE
	.p2align	4, 0x90
_ZN3std2rt10lang_start17h50d31464143d65edE:
.seh_proc _ZN3std2rt10lang_start17h50d31464143d65edE
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%r8, %rax
	movq	%rdx, %r8
	movq	%rcx, 48(%rsp)
	movb	%r9b, 32(%rsp)
	leaq	__unnamed_1(%rip), %rdx
	leaq	48(%rsp), %rcx
	movq	%rax, %r9
	callq	_ZN3std2rt19lang_start_internal17h522c822308cc94e9E
	nop
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3269913c9eb9b3deE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3269913c9eb9b3deE
	.p2align	4, 0x90
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3269913c9eb9b3deE:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3269913c9eb9b3deE
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h38be00a484bb4f80E
	xorl	%eax, %eax
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7e499bd8718b3070E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7e499bd8718b3070E
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7e499bd8718b3070E:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7e499bd8718b3070E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h38be00a484bb4f80E
	xorl	%eax, %eax
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc2d553d793ea1bb3E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc2d553d793ea1bb3E
	.p2align	4, 0x90
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc2d553d793ea1bb3E:
	retq

	.def	_ZN8rust1_664main17h84a192c9a3e8cb4dE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN8rust1_664main17h84a192c9a3e8cb4dE
	.p2align	4, 0x90
_ZN8rust1_664main17h84a192c9a3e8cb4dE:
.seh_proc _ZN8rust1_664main17h84a192c9a3e8cb4dE
	subq	$88, %rsp
	.seh_stackalloc 88
	.seh_endprologue
	leaq	__unnamed_2(%rip), %rax
	movq	%rax, 40(%rsp)
	movq	$1, 48(%rsp)
	movq	$0, 56(%rsp)
	leaq	__unnamed_3(%rip), %rax
	movq	%rax, 72(%rsp)
	movq	$0, 80(%rsp)
	leaq	40(%rsp), %rcx
	callq	_ZN3std2io5stdio6_print17hd82ca23f1d065287E
	nop
	addq	$88, %rsp
	retq
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,main
	.globl	main
	.p2align	4, 0x90
main:
.seh_proc main
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%rdx, %r9
	movslq	%ecx, %r8
	leaq	_ZN8rust1_664main17h84a192c9a3e8cb4dE(%rip), %rax
	movq	%rax, 48(%rsp)
	movb	$0, 32(%rsp)
	leaq	__unnamed_1(%rip), %rdx
	leaq	48(%rsp), %rcx
	callq	_ZN3std2rt19lang_start_internal17h522c822308cc94e9E
	nop
	addq	$56, %rsp
	retq
	.seh_endproc

	.section	.rdata,"dr",one_only,__unnamed_1
	.p2align	3
__unnamed_1:
	.quad	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc2d553d793ea1bb3E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7e499bd8718b3070E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3269913c9eb9b3deE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h3269913c9eb9b3deE

	.section	.rdata,"dr",one_only,__unnamed_3
	.p2align	3
__unnamed_3:

	.section	.rdata,"dr",one_only,__unnamed_4
__unnamed_4:
	.ascii	"Hello, world!\n"

	.section	.rdata,"dr",one_only,__unnamed_2
	.p2align	3
__unnamed_2:
	.quad	__unnamed_4
	.asciz	"\016\000\000\000\000\000\000"

