--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email character(50) NOT NULL,
    c_mobile character varying(30) NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character(50) NOT NULL,
    data_duration integer NOT NULL,
    data_price real NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation character(50) NOT NULL,
    pnumber integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration character(50) NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    staff_sal real NOT NULL,
    age integer NOT NULL,
    mobile character varying(15)
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	MUSTA KARIM	35	m_karim@gmail.com                                 	8055089112	102	5
111	LILIAN JAIYA	43	l_jaiye@gmail.com                                 	8055185341	100	3
112	ARTHUR MUSA	50	a_musa@gmail.com                                  	7055282813	107	10
113	PHILIP AKONJO	41	p_akonjo@gmail.com                                	9052356772	100	2
114	MARYLENE MAPA	33	m_mapa@gmail.com                                  	8053333551	120	5
115	OGHENERO AGOR	50	o_agor@gmail.com                                  	7055566774	117	11
116	ADAMS BREE	33	a_bree@gmail.com                                  	8056765424	102	1
117	OKAFOR MATHIAS	45	o_mathias@gmail.com                               	8056763367	120	10
118	SAMSON ADELEKE	65	s_adeleke@gmail.com                               	7056774423	117	11
119	LAWAL TAMIRE	35	l_tamire@gmail.com                                	9052111101	107	5
120	JAMES JOB	44	j_job@gmail.com                                   	8059693919	100	8
121	MATTHEW JAKANDE	21	m_jakande@gmail.com                               	7051232144	120	5
122	JIMILA ADEGBOYE	20	j_adegboye@gmail.com                              	8054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB                                             	2	200
2	1.8GB                                             	14	500
3	3.9GB                                             	30	1000
4	7.5GB                                             	30	1500
5	9.2GB                                             	30	2000
6	10.8GB                                            	30	2500
7	14GB                                              	30	3000
8	18GB                                              	30	4000
9	24GB                                              	30	5000
10	29.9GB                                            	30	8000
11	50GB                                              	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pnumber) FROM stdin;
108	1	Administration	Ikeja                                             	44
101	2	Account	Egbeda                                            	11
100	3	Packaging	Ajah                                              	44
120	4	Research	V.I                                               	33
97	5	Account	Magodo                                            	22
122	6	Operations	Mile 2                                            	44
107	7	Packaging	Ketu                                              	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 MONTHS                                          	102
22	B	14 MONTHS                                         	97
33	C	16 MONTHS                                         	120
44	D	25 MONTHS                                         	108
55	E	9 MONTHS                                          	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
101	ALADE JOY	2	250000	33	8023089832
100	MUSTAPHA ALI	3	175000	33	8063285831
107	ALOKWE MARTIN	7	38000	48	7090082812
97	DANKADE AMINAT	5	550000	40	9023688832
108	JOSIAH JOSHUA	1	120000	30	8053189131
102	MANKINDE MARY	2	450000	55	9023487830
120	ADELEKE JANE	4	200000	38	7061045862
122	OSAHON MARK	6	320000	44	8022289842
177	SULEMAN AJAYI	3	800000	50	7030089811
104	KUTI LAWAL	1	750000	35	9145689842
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

