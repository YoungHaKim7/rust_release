	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"rust1_65vs_66.2720d0b0-cgu.0"
	.def	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha70c73ba67e1ad4cE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha70c73ba67e1ad4cE
	.p2align	4, 0x90
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha70c73ba67e1ad4cE:
.seh_proc _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha70c73ba67e1ad4cE
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

	.def	_ZN3std2rt10lang_start17h409197884dd5984fE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17h409197884dd5984fE
	.globl	_ZN3std2rt10lang_start17h409197884dd5984fE
	.p2align	4, 0x90
_ZN3std2rt10lang_start17h409197884dd5984fE:
.seh_proc _ZN3std2rt10lang_start17h409197884dd5984fE
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
	callq	_ZN3std2rt19lang_start_internal17h9b461f8940399158E
	nop
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8ae057fbfa7ba1e6E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8ae057fbfa7ba1e6E
	.p2align	4, 0x90
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8ae057fbfa7ba1e6E:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8ae057fbfa7ba1e6E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha70c73ba67e1ad4cE
	xorl	%eax, %eax
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h98928eb7901a6336E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h98928eb7901a6336E
	.p2align	4, 0x90
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h98928eb7901a6336E:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h98928eb7901a6336E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha70c73ba67e1ad4cE
	xorl	%eax, %eax
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hee85f35b09943d67E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hee85f35b09943d67E
	.p2align	4, 0x90
_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hee85f35b09943d67E:
	retq

	.def	_ZN13rust1_65vs_664main17hf6208302001efb97E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN13rust1_65vs_664main17hf6208302001efb97E
	.p2align	4, 0x90
_ZN13rust1_65vs_664main17hf6208302001efb97E:
.seh_proc _ZN13rust1_65vs_664main17hf6208302001efb97E
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
	callq	_ZN3std2io5stdio6_print17hf9f676e81421b601E
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
	leaq	_ZN13rust1_65vs_664main17hf6208302001efb97E(%rip), %rax
	movq	%rax, 48(%rsp)
	movb	$2, 32(%rsp)
	leaq	__unnamed_1(%rip), %rdx
	leaq	48(%rsp), %rcx
	callq	_ZN3std2rt19lang_start_internal17h9b461f8940399158E
	nop
	addq	$56, %rsp
	retq
	.seh_endproc

	.section	.rdata,"dr",one_only,__unnamed_1
	.p2align	3
__unnamed_1:
	.quad	_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hee85f35b09943d67E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h98928eb7901a6336E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8ae057fbfa7ba1e6E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h8ae057fbfa7ba1e6E

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

