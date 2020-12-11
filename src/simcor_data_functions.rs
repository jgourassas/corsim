use std::collections::HashMap;
/*** ***********************************************************/
pub fn get_segments_names_92() -> Vec<String> {
    let segment_names: Vec<String> = vec![
        "LM".to_string(),
        "L1".to_string(),
        "L2".to_string(),
        "L3".to_string(),
        "L4".to_string(),
        "C1".to_string(),
        "C2".to_string(),
        "C3".to_string(),
        "C4".to_string(),
        "R1".to_string(),
        "R2".to_string(),
        "R3".to_string(),
        "R4".to_string(),
        "S1".to_string(),
        "S2".to_string(),
        "S3".to_string(),
        "D1".to_string(),
        "D2".to_string(),
        "D3".to_string(),
        "MR".to_string(),
        "OM".to_string(),//?Obtuse marginal
        "OA".to_string(),//Anterior distal branch of OM
        "OP".to_string(),//Posterior distal branch of OM
        "M1".to_string(),
        "M2".to_string(),
        "M3".to_string(),
        "CP".to_string(),
        "CI".to_string(),
        //"CD".to_string(),//Distal most inferior wall branch arises from C4, present only inleft-dominant anatomy
      
        "RD".to_string(),
        "RI".to_string(),
        //"RP".to_string(),
    
        ];
    segment_names
} //get segments_names
  // Fom dodge 92  RCA Dominant
  // midpoints_diameters
pub fn get_midpoint_92(point_name: &str) -> Vec<f32> {
    let mut points = HashMap::new();
    points.insert(
        String::from("LMp"),
        vec![
            0.29554435894139947,
            0.03656080302154424,
            0.036288284339950155,
        ],
    );
    points.insert(
        String::from("LMm"),
        vec![0.6876861675754944, 0.04882953162088771, 0.12125762560356244],
    );
    points.insert(
        String::from("LMd"),
        vec![1.0797899017924304, 0.0, 0.20988989491419943],
    );
    points.insert(
        String::from("L1p"),
        vec![1.5014476875323806, -0.08373752998871013, 0.5464822665696085],
    );
    points.insert(
        String::from("L1m"),
        vec![2.041516743331695, -0.20045820831961378, 1.0402047362964846],
    );
    points.insert(
        String::from("L1d"),
        vec![2.5617165610111714, -0.2614672282429745, 1.5392345986223264],
    );
    points.insert(
        String::from("L2p"),
        vec![3.0703120202865937, -0.6257378601609235, 2.486289650954789],
    );
    points.insert(
        String::from("L2m"),
        vec![3.601235617488854, -1.1019319613341243, 3.7291886490120087],
    );
    points.insert(
        String::from("L2d"),
        vec![3.911843539405599, -1.900416080697789, 4.830724463621986],
    );
    points.insert(
        String::from("L3p"),
        vec![4.010743000887889, -3.1258490279141897, 6.17600262593182],
    );
    points.insert(
        String::from("L3m"),
        vec![4.282642885589951, -5.352184568755369, 7.4177550685151825],
    );
    points.insert(
        String::from("L3d"),
        vec![4.655596252298606, -7.584893794301164, 7.748213320808538],
    );
    points.insert(
        String::from("L4p"),
        vec![4.490976801318241, -8.775945564131543, 7.187065244271911],
    );
    points.insert(
        String::from("L4m"),
        vec![4.21632120625497, -8.90557304662871, 6.492565301700468],
    );
    points.insert(
        String::from("L4d"),
        vec![3.924718218543799, -8.98332259111663, 5.81863404322389],
    );
    points.insert(
        String::from("C1p"),
        vec![1.3660708352759223, -0.291076367144863, 0.09552497839985384],
    );
    points.insert(
        String::from("C1m"),
        vec![
            1.6403761922803517,
            -0.7321259575364403,
            -0.11470627751419989,
        ],
    );
    points.insert(
        String::from("C1d"),
        vec![1.8445595049403363, -1.1330837648021193, -0.3920732261658757],
    );
    points.insert(
        String::from("C2p"),
        vec![2.085264097312316, -1.4160614910390705, -0.6375292134925724],
    );
    points.insert(
        String::from("C2m"),
        vec![2.2252074089401765, -1.8656265717713496, -1.085306169658326],
    );
    points.insert(
        String::from("C2d"),
        vec![2.3529516082286777, -2.361812504365826, -1.3584772444009892],
    );
    points.insert(
        String::from("C3p"),
        vec![2.3454517515525057, -2.8991378028648445, -1.7040704448875588],
    );
    points.insert(
        String::from("C3m"),
        vec![2.091484565436457, -3.652586018847763, -2.0914845654364567],
    );
    points.insert(
        String::from("C3d"),
        vec![1.9003220820963247, -4.476802891797226, -2.346702335681518],
    );
    points.insert(
        String::from("C4p"),
        vec![0.9058706050175953, -5.24687766467891, -2.0346186912683786],
    );
    points.insert(
        String::from("C4m"),
        vec![0.4646961839316486, -5.578559596035969, -1.8637945943999221],
    );
    points.insert(
        String::from("C4d"),
        vec![1.6005231360110088, -5.9290894869708195, -0.8510132470094607],
    );
    points.insert(
        String::from("R1p"),
        vec![-0.43833477874433646, -0.187303296707956, 0.1509307681868909],
    );
    points.insert(
        String::from("R1m"),
        vec![-1.439941817856379, -0.6914522932288603, 0.5817742580907276],
    );
    points.insert(
        String::from("R1d"),
        vec![-2.0569992919589706, -1.2999999999999998, 0.9158350904394823],
    );
    points.insert(
        String::from("R2p"),
        vec![-2.466796077701761, -2.139689329569447, 0.9469142959961353],
    );
    points.insert(
        String::from("R2m"),
        vec![-2.927352707778549, -3.418466197196013, 0.9511545525779821],
    );
    points.insert(
        String::from("R2d"),
        vec![-3.129703972901258, -4.587251448018354, 0.7225490949822015],
    );
    points.insert(
        String::from("R3p"),
        vec![-2.894778440348643, -5.474275075725346, 0.30425347423641175],
    );
    points.insert(
        String::from("R3m"),
        vec![-2.293205557573059, -5.974914729582092, -0.04002805190118708],
    );
    points.insert(
        String::from("R3d"),
        vec![0.5069981483899437, -6.407241864952337, 1.8921428491346153],
    );
    points.insert(
        String::from("R4p"),
        vec![1.3811897602375875, -6.24042259895549, -1.6460378588174798],
    );
    points.insert(
        String::from("R4m"),
        vec![-0.04401090588894932, -5.663981837384125, -2.521383109712917],
    );
    points.insert(
        String::from("R4d"),
        vec![0.5282419597121412, -5.1997575212616285, -3.3351884732474417],
    );

    /******************************BRANCHES*********************************************/
    points.insert(
        String::from("S1o"),
        vec![2.7335389671479917, -0.5162337346327618, 1.7751809615671865],
    );
    points.insert(
        String::from("S1p"),
        vec![2.7105161936648146, -1.081775307474126, 2.274393138404191],
    );
    points.insert(
        String::from("S1m"),
        vec![2.114099152481461, -2.2146637221132326, 3.0192464906715926],
    );
    points.insert(
        String::from("S2o"),
        vec![3.465339933810029, -1.0603496231705725, 3.588464549049291],
    );
    points.insert(
        String::from("S2p"),
        vec![3.240034927366695, -1.6377900701872212, 3.8613232648199793],
    );
    points.insert(
        String::from("S2m"),
        vec![2.6126822114121246, -2.811895797428755, 4.348233398355378],
    );
    points.insert(
        String::from("S3o"),
        vec![3.9262882559084624, -2.246420265754381, 5.210360498192356],
    );
    points.insert(
        String::from("S3p"),
        vec![3.603178471626892, -2.735117899424916, 5.341931764619418],
    );
    points.insert(
        String::from("S3m"),
        vec![3.1286606475627097, -3.7597779468433954, 5.418999201219959],
    );
    points.insert(
        String::from("D1o"),
        vec![2.771545498471191, -0.44535392307220945, 1.536292756315758],
    );
    points.insert(
        String::from("D1p"),
        vec![3.5792536949729974, -0.9447944282442331, 1.984012720569301],
    );
    points.insert(
        String::from("D1m"),
        vec![4.991568449151188, -2.422532996633497, 2.766871572310565],
    );
    points.insert(
        String::from("D2o"),
        vec![3.505322155780238, -1.0187672850070206, 3.2687657920376627],
    );
    points.insert(
        String::from("D2p"),
        vec![4.147831567340345, -1.626260399320295, 3.8679155113028947],
    );
    points.insert(
        String::from("D2m"),
        vec![5.308853618159379, -3.131872151683661, 4.614916043273639],
    );
    points.insert(
        String::from("D3o"),
        vec![4.1180648163315725, -2.0395121628746526, 4.737291663596232],
    );
    points.insert(
        String::from("D3p"),
        vec![4.532380623883003, -2.651922826635222, 5.213907479215658],
    );
    points.insert(
        String::from("D3m"),
        vec![5.516784590489722, -3.9453403211016966, 5.916027174787512],
    );
    points.insert(
        String::from("MRo"),
        vec![1.1772723146284882, -0.2083778132003164, 0.10299798141291751],
    );
    points.insert(
        String::from("MRp"),
        vec![2.5336929727108926, -0.9234543869793056, 0.13278522208482535],
    );
    points.insert(
        String::from("MRm"),
        vec![4.572078722347505, -3.1314802594361826, 0.806180836672952],
    );
    points.insert(String::from("OMp"), vec![2.342245650465862, -1.3999999999999997, -0.6276028305176377]);
    points.insert(String::from("OMm"), vec![2.8384734967392573, -1.8547174248162173, -0.8678084364604395]);
    points.insert(String::from("OMd"), vec![3.1720338774402985, -2.4099195343991395, -0.9697880789597789]);
    points.insert(String::from("OAo"), vec![3.3604513207724396, -2.6479861018690127, -1.027393072308413]);
    points.insert(String::from("OAp"), vec![3.79147090319087, -3.6145913083312418, -0.8059020188541839]);
    points.insert(String::from("OAm"), vec![4.632501989794268, -5.358438019581681, 0.48689557900405106]);
    
    /*
    points.insert(
        String::from("OMp"),
        vec![2.342245650465862, -1.3999999999999997, -0.6276028305176377],
    );
    points.insert(
        String::from("OMm"),
        vec![2.8384734967392573, -1.8547174248162173, -0.8678084364604395],
    );
    points.insert(
        String::from("OMd"),
        vec![3.1720338774402985, -2.4099195343991395, -0.9697880789597789],
    );
    points.insert(
        String::from("OAo"),
        vec![3.3604513207724396, -2.6479861018690127, -1.027393072308413],
    );
    points.insert(
        String::from("OAp"),
        vec![3.79147090319087, -3.6145913083312418, -0.8059020188541839],
    );
    points.insert(
        String::from("OAm"),
        vec![4.632501989794268, -5.358438019581681, 0.48689557900405106],
    );
    */

    points.insert(
        String::from("OPo"),
        vec![3.3604513207724396, -2.6479861018690127, -1.027393072308413],
    );
    points.insert(
        String::from("OPp"),
        vec![3.2738117750170477, -3.3417919643062426, -1.4575949125384446],
    );
    points.insert(
        String::from("OPm"),
        vec![3.407718886629475, -5.031403713297944, -1.6620555536805697],
    );
    points.insert(
        String::from("M1o"),
        vec![1.8445595049403363, -1.1330837648021193, -0.3920732261658757],
    );
    points.insert(
        String::from("M1p"),
        vec![3.0335387554850115, -1.6431504697506178, -0.5896602010724263],
    );
    points.insert(
        String::from("M1m"),
        vec![4.412188120211939, -3.4477042618236857, -0.07701503011380952],
    );
    points.insert(
        String::from("M2o"),
        vec![2.265654624361911, -2.2306006985677245, -1.204669932660672],
    );
    points.insert(
        String::from("M2p"),
        vec![2.841286757641696, -3.0007927842749935, -1.5107389670929507],
    );
    points.insert(
        String::from("M2m"),
        vec![3.570204718446285, -4.6628757687418245, -1.2293200696581095],
    );
    points.insert(
        String::from("M3o"),
        vec![2.36208075552619, -3.5471350270470285, -1.9820210907729772],
    );
    points.insert(
        String::from("M3p"),
        vec![2.5358317946220725, -4.176456994115626, -2.053476098153194],
    );
    points.insert(
        String::from("M3m"),
        vec![3.041291101967833, -5.512312625016769, -1.6170831633444278],
    );
    points.insert(
        String::from("CPo"),
        vec![1.275159295210584, -5.392764277795003, -2.3004482641381436],
    );
    points.insert(
        String::from("CPp"),
        vec![1.2312963651384998, -5.983281547440861, -2.2213174436019734],
    );
    points.insert(
        String::from("CPm"),
        vec![1.419210485854215, -7.185941174554807, -2.026842626520203],
    );
    points.insert(
        String::from("CPo"),
        vec![1.275159295210584, -5.392764277795003, -2.3004482641381436],
    );
    points.insert(
        String::from("CIo"),
        vec![2.2629359004682725, -5.983281547440861, -1.1530234318631762],
    );
    points.insert(
        String::from("CIp"),
        vec![2.235157267734093, -6.535062985480412, -1.138869511532953],
    );
    points.insert(
        String::from("CIm"),
        vec![2.0457124380835676, -7.6504380477042835, -1.1339569210992335],
    );
    points.insert(
        String::from("CDo"),
        vec![-1.6152216849229866, -6.440453362786736, -0.8953320000556038],
    );
    points.insert(
        String::from("CDp"),
        vec![1.9398856050425095, -7.11333654994356, 0.6303070415169999],
    );
    points.insert(
        String::from("CDm"),
        vec![-0.7806259699282441, -8.592888563177699, 1.1148494229641517],
    );
    points.insert(
        String::from("RDo"),
        vec![1.8921428491346153, -6.407241864952337, -0.506998148389944],
    );
    points.insert(
        String::from("RDp"),
        vec![1.5342990137617705, -6.820590453496647, -0.35422083795291515],
    );
    points.insert(
        String::from("RDm"),
        vec![1.8106738439157182, -7.3742475196975725, 0.31927065194544907],
    );
    points.insert(
        String::from("RIo"),
        vec![0.19951329116353397, -5.613341102386717, -2.853172990707883],
    );
    points.insert(
        String::from("RIp"),
        vec![0.5137082370002675, -6.3441545092565494, -2.9133841850874838],
    );
    points.insert(
        String::from("RIm"),
        vec![1.7133734552402526, -7.046597294707584, -2.2737233712784475],
    );
    points.insert(
        String::from("RPo"),
        vec![0.8136827674910369, -4.905612271448018, -4.186034949221846],
    );
    points.insert(
        String::from("RPp"),
        vec![1.0872765354568406, -5.752478501329071, -4.360827997891206],
    );
    points.insert(
        String::from("RPm"),
        vec![2.565918982444365, -6.632300580440334, -3.6645120802396716],
    );

    let point = point_name.trim().to_string();
    let point_vec = points.get(&point);

    if let Some(point_vec) = point_vec {
        point_vec.to_vec()
    } else {
        vec![0.0, 0.0, 0.0]
    }
}

/***********************************************************************8 */
pub fn get_segment_points_92(segment_name: &str) -> Vec<&str> {
    let mut segments = HashMap::new();
    segments.insert(String::from("LM"), vec!["LMp", "LMm", "LMd"]);
    segments.insert(String::from("L1"), vec!["LMd", "L1p", "L1m", "L1d"]);
    segments.insert(String::from("L2"), vec!["L1d", "L2p", "L2m", "L2d"]);
    segments.insert(String::from("L3"), vec!["L2d", "L3p", "L3m", "L3d"]);
    segments.insert(String::from("L4"), vec!["L3d", "L4p", "L4m", "L4d"]);

    segments.insert(String::from("C1"), vec!["LMd", "C1p", "C1m", "C1d"]);
    segments.insert(String::from("C2"), vec!["C1d", "C2p", "C2m", "C2d"]);
    segments.insert(String::from("C3"), vec!["C2d", "C3p", "C3m", "C3d"]);
    segments.insert(String::from("C4"), vec!["C3d", "C4p", "C4m", "C4d"]);

    segments.insert(String::from("R1"), vec!["R1p", "R1m", "R1d"]);
    segments.insert(String::from("R2"), vec!["R1d", "R2p", "R2m", "R2d"]);
    segments.insert(String::from("R3"), vec!["R2d", "R3p", "R3m", "R3d"]);
    segments.insert(String::from("R4"), vec!["R3d", "R4p", "R4m", "R4d"]);
    segments.insert(String::from("S1"), vec!["L1d", "S1o", "S1p", "S1m"]);
    segments.insert(String::from("S2"), vec!["L2m", "S2o", "S2p", "S2m"]);
    segments.insert(String::from("S3"), vec!["L2d", "S3o", "S3p", "S3m"]);
    segments.insert(String::from("D1"), vec!["L1d", "D1o", "D1p", "D1m"]);
    segments.insert(String::from("D2"), vec!["L2m", "D2o", "D2p", "D2m"]);
    segments.insert(String::from("D3"), vec!["L2d", "D3o", "D3p", "D3m"]);
    segments.insert(String::from("MR"), vec!["C1p", "MRo", "MRp", "MRm"]); //Ramus Medianus
    segments.insert(String::from("OM"), vec!["C1d", "OMp", "OMm", "OMd"]); //Obtuse marginal
    segments.insert(String::from("OA"), vec!["OMd", "OAo", "OAp", "OAm"]); //Anterior branch Obtuse marginal
    segments.insert(String::from("OP"), vec!["OMd", "OPo", "OPp", "OPm"]); //POSTERIOR branch Obtuse marginal
    segments.insert(String::from("M1"), vec!["C1d", "M1o", "M1p", "M1m"]); //M1 MARGINAL  FROM LCX
    segments.insert(String::from("M2"), vec!["C2m", "M2o", "M2p", "M2m"]); //M2 MARGINAL FROM LCX
    segments.insert(String::from("M3"), vec!["C3p", "M3o", "M3p", "M3m"]); //M3 MARGINAL FROM LCX
                                                                           /* CP = Proximal most inferior wall branch arises from junction of C3 and C4,
                                                                            present in small-right, balanced, and left-dominant anatomy
                                                                           */
    segments.insert(String::from("CP"), vec!["C3d", "CPo", "CPp", "CPm"]);
    /*
       CI Inferior Inferior wall branch arises from C4, present only in balanced and
                  left-dominant anatomy
    */
    segments.insert(String::from("CI"), vec!["C4d", "CIo", "CIp", "CIm"]);

    /*
    CD Posterior descending Distal most inferior wall branch arises from C4, present only in
                              left-dominant anatomy
    */
    segments.insert(String::from("CD"), vec!["C4d", "CDo", "CDp", "CDm"]);
    /*
    RD Posterior descending Proximal most inferior wall branch arises from junction of R3 and R4,
                              present in right, small-right, and balanced-dominant anatomy

    */
    segments.insert(String::from("RD"), vec!["R3d", "RDo", "RDp", "RDm"]);
    /*
    RI Inferior Inferior wall branch arises from R4, present only in right and
                  small-right-dominant anatomy

    */
    segments.insert(String::from("RI"), vec!["R4m", "RIo", "RIp", "RIm"]);
    /*
    RP Posterior Distal most inferior wall branch arises from R4, present only in
                   right-dominant anatomy
    */
    segments.insert(String::from("RP"), vec!["R4d", "RPo", "RPp", "RPm"]);

    let segment = segment_name.trim().to_string();
    let point_names = segments.get(&segment);

    if let Some(point_names) = point_names {
        point_names.to_vec()
    } else {
        vec!["R", "R", "R"]
    }
}

//The parameters to glColor3f have to be floating point values in the range [0.0, 1.0],
//compare to glColor3ub, where the parameters are integral values in the range [0, 255].
//['rgb(215,25,28)','rgb(253,174,97)','rgb(166,217,106)','rgb(26,150,65)']
pub fn get_midpoint_color_92(point_name: &str) -> Vec<u8> {
    let mut colors = HashMap::new();
    colors.insert(String::from("LMp"), vec![215, 25, 28]);
    colors.insert(String::from("LMm"), vec![215, 25, 28]);
    colors.insert(String::from("LMd"), vec![215, 25, 28]);

    colors.insert(String::from("L1p"), vec![166, 217, 106]);
    colors.insert(String::from("L1m"), vec![166, 217, 106]);
    colors.insert(String::from("L1d"), vec![166, 217, 106]);

    colors.insert(String::from("L2p"), vec![166, 217, 106]);
    colors.insert(String::from("L2m"), vec![166, 217, 106]);
    colors.insert(String::from("L2d"), vec![166, 217, 106]);

    colors.insert(String::from("L3p"), vec![166, 217, 106]);
    colors.insert(String::from("L3m"), vec![166, 217, 106]);
    colors.insert(String::from("L3d"), vec![166, 217, 106]);

    colors.insert(String::from("L4p"), vec![166, 217, 106]);
    colors.insert(String::from("L4m"), vec![166, 217, 106]);
    colors.insert(String::from("L4d"), vec![166, 217, 106]);

    colors.insert(String::from("C1p"), vec![255, 255, 191]);
    colors.insert(String::from("C1m"), vec![255, 255, 191]);
    colors.insert(String::from("C1d"), vec![255, 255, 191]);

    colors.insert(String::from("C2p"), vec![255, 255, 191]);
    colors.insert(String::from("C2m"), vec![255, 255, 191]);
    colors.insert(String::from("C2d"), vec![255, 255, 191]);

    colors.insert(String::from("C3p"), vec![255, 255, 191]);
    colors.insert(String::from("C3m"), vec![255, 255, 191]);
    colors.insert(String::from("C3d"), vec![255, 255, 191]);

    colors.insert(String::from("C4p"), vec![255, 255, 191]);
    colors.insert(String::from("C4m"), vec![255, 255, 191]);
    colors.insert(String::from("C4d"), vec![255, 255, 191]);

    colors.insert(String::from("R1p"), vec![253, 174, 97]);
    colors.insert(String::from("R1m"), vec![253, 174, 97]);
    colors.insert(String::from("R1d"), vec![253, 174, 97]);

    colors.insert(String::from("R2p"), vec![253, 174, 97]);
    colors.insert(String::from("R2m"), vec![253, 174, 97]);
    colors.insert(String::from("R2d"), vec![253, 174, 97]);

    colors.insert(String::from("R3p"), vec![253, 174, 97]);
    colors.insert(String::from("R3m"), vec![253, 174, 97]);
    colors.insert(String::from("R3d"), vec![253, 174, 97]);

    colors.insert(String::from("R4p"), vec![253, 174, 97]);
    colors.insert(String::from("R4m"), vec![253, 174, 97]);
    colors.insert(String::from("R4d"), vec![253, 174, 97]);

    colors.insert(String::from("S1p"), vec![166, 217, 106]);
    colors.insert(String::from("S1m"), vec![166, 217, 106]);
    colors.insert(String::from("S1d"), vec![166, 217, 106]);

    colors.insert(String::from("S2p"), vec![166, 217, 106]);
    colors.insert(String::from("S2m"), vec![166, 217, 106]);
    colors.insert(String::from("S2d"), vec![166, 217, 106]);

    colors.insert(String::from("S3p"), vec![166, 217, 106]);
    colors.insert(String::from("S3m"), vec![166, 217, 106]);
    colors.insert(String::from("S3d"), vec![166, 217, 106]);

    colors.insert(String::from("D1p"), vec![166, 217, 106]);
    colors.insert(String::from("D1m"), vec![166, 217, 106]);
    colors.insert(String::from("D1d"), vec![166, 217, 106]);

    colors.insert(String::from("D2p"), vec![166, 217, 106]);
    colors.insert(String::from("D2m"), vec![166, 217, 106]);
    colors.insert(String::from("D2d"), vec![215, 25, 28]);

    colors.insert(String::from("D3p"), vec![166, 217, 106]);
    colors.insert(String::from("D3m"), vec![166, 217, 106]);
    colors.insert(String::from("D3d"), vec![215, 25, 28]);

    colors.insert(String::from("MRp"), vec![203, 201, 226]);
    colors.insert(String::from("MRm"), vec![203, 201, 226]);
    colors.insert(String::from("MRd"), vec![203, 201, 226]);

    colors.insert(String::from("OMp"), vec![255, 255, 191]);
    colors.insert(String::from("OMm"), vec![255, 255, 191]);
    colors.insert(String::from("OMd"), vec![255, 255, 191]);

//    segments.insert(String::from("M3"), vec!["C3p", "M3o", "M3p", "M3m"]); //M3 MARGINAL FROM LCX
    


    colors.insert(String::from("OAo"), vec![255, 255, 191]);
    colors.insert(String::from("OAp"), vec![255, 255, 191]);
    colors.insert(String::from("OAm"), vec![255, 255, 191]);
    

    colors.insert(String::from("M1p"), vec![255, 255, 191]);
    colors.insert(String::from("M1m"), vec![255, 255, 191]);
    colors.insert(String::from("M1d"), vec![255, 255, 191]);

    colors.insert(String::from("M2p"), vec![255, 255, 191]);
    colors.insert(String::from("M2m"), vec![255, 255, 191]);
    colors.insert(String::from("M2d"), vec![255, 255, 191]);

    colors.insert(String::from("M3o"), vec![255, 255, 191]);
    colors.insert(String::from("M3p"), vec![255, 255, 191]);
    colors.insert(String::from("M3m"), vec![255, 255, 191]);

    colors.insert(String::from("CPp"), vec![171, 217, 233]);
    colors.insert(String::from("CPm"), vec![171, 217, 233]);
    colors.insert(String::from("CPd"), vec![171, 217, 233]);

    colors.insert(String::from("CIo"), vec![215, 25, 28]);
    colors.insert(String::from("CIp"), vec![215, 25, 28]);
    colors.insert(String::from("CIm"), vec![215, 25, 28]);

    colors.insert(String::from("RDo"), vec![253, 174, 97]);
    colors.insert(String::from("RDp"), vec![253, 174, 97]);
    colors.insert(String::from("RDm"), vec![253, 174, 97]);

    colors.insert(String::from("RIo"), vec![215, 25, 28]);
    colors.insert(String::from("RIp"), vec![253, 174, 97]);
    colors.insert(String::from("RIm"), vec![215, 25, 28]);

    colors.insert(String::from("RPo"), vec![253, 174, 97]);
    colors.insert(String::from("RPp"), vec![253, 174, 97]);
    colors.insert(String::from("RPm"), vec![253, 174, 97]);

    let point = point_name.trim().to_string();
    let color = colors.get(&point);

    if let Some(color) = color {
        color.to_vec()
    } else {
        vec![166, 97, 26]
    }
    /*
        let point = point_name.trim().to_string();
        let color_vec = colors.get(&point);

        if let Some(color_vec) = color_vec {
            color_vec.to_vec()
        } else {
            vec!(166.0, 97.0, 26.0)
           }
    */
}

pub fn get_diameter(point_name: &str) -> f32 {
    let mut diameters = HashMap::new();

    diameters.insert(String::from("LMp"), 4.6);
    diameters.insert(String::from("LMm"), 4.5);
    diameters.insert(String::from("LMd"), 4.5);

    diameters.insert(String::from("L1p"), 3.7);
    diameters.insert(String::from("L1m"), 3.6);
    diameters.insert(String::from("L1d"), 3.5);

    diameters.insert(String::from("L2p"), 2.9);
    diameters.insert(String::from("L2m"), 2.5);
    diameters.insert(String::from("L2d"), 2.3);

    diameters.insert(String::from("L3p"), 2.0);
    diameters.insert(String::from("L3m"), 1.7);
    diameters.insert(String::from("L3d"), 1.4);

    diameters.insert(String::from("L4p"), 1.4);
    diameters.insert(String::from("L4m"), 1.1);
    diameters.insert(String::from("L4d"), 0.9);

    diameters.insert(String::from("C1p"), 3.4);
    diameters.insert(String::from("C1m"), 3.4);
    diameters.insert(String::from("C1d"), 3.3);

    diameters.insert(String::from("C2p"), 2.8);
    diameters.insert(String::from("C2m"), 2.8);
    diameters.insert(String::from("C2d"), 2.7);

    diameters.insert(String::from("C3p"), 1.7);
    diameters.insert(String::from("C3m"), 1.6);
    diameters.insert(String::from("C3d"), 1.3);

    diameters.insert(String::from("C4p"), 1.2); // Is Empty in the Paper
    diameters.insert(String::from("C4m"), 1.1); // Is Empty in the Paper
    diameters.insert(String::from("C4d"), 1.0); // Is Empty in the Paper

    diameters.insert(String::from("R1p"), 4.0);
    diameters.insert(String::from("R1m"), 3.9);
    diameters.insert(String::from("R1d"), 3.8);

    diameters.insert(String::from("R2p"), 3.5);
    diameters.insert(String::from("R2m"), 3.4);
    diameters.insert(String::from("R2d"), 3.2);

    diameters.insert(String::from("R3p"), 3.2);
    diameters.insert(String::from("R3m"), 3.1);
    diameters.insert(String::from("R3d"), 3.1);

    diameters.insert(String::from("R4p"), 2.4);
    diameters.insert(String::from("R4m"), 2.2);
    diameters.insert(String::from("R4d"), 1.9);

    diameters.insert(String::from("S1o"), 1.8);
    diameters.insert(String::from("S1p"), 1.8);
    diameters.insert(String::from("S1m"), 1.1);
    diameters.insert(String::from("S2o"), 1.9);
    diameters.insert(String::from("S2p"), 1.7);
    diameters.insert(String::from("S2m"), 1.4);
    diameters.insert(String::from("S3o"), 1.4);
    diameters.insert(String::from("S3p"), 1.3);
    diameters.insert(String::from("S3m"), 1.2);
    diameters.insert(String::from("D1o"), 2.4);
    diameters.insert(String::from("D1p"), 2.3);
    diameters.insert(String::from("D1m"), 1.8);
    diameters.insert(String::from("D2o"), 2.6);
    diameters.insert(String::from("D2p"), 2.4);
    diameters.insert(String::from("D2m"), 1.9);
    diameters.insert(String::from("D3o"), 2.4);
    diameters.insert(String::from("D3p"), 2.3);
    diameters.insert(String::from("D3m"), 1.7);
    diameters.insert(String::from("MRo"), 2.7);
    diameters.insert(String::from("MRp"), 2.4);
    diameters.insert(String::from("MRm"), 2.0);
    diameters.insert(String::from("OMp"), 3.3);
    diameters.insert(String::from("OMm"), 3.0);
    diameters.insert(String::from("OMd"), 2.7);
    diameters.insert(String::from("OAo"), 2.5);
    diameters.insert(String::from("OAp"), 2.4);
    diameters.insert(String::from("OAm"), 2.0);
    diameters.insert(String::from("OPo"), 2.5);
    diameters.insert(String::from("OPp"), 2.3);
    diameters.insert(String::from("OPm"), 1.9);
    diameters.insert(String::from("M1o"), 2.6);
    diameters.insert(String::from("M1p"), 2.5);
    diameters.insert(String::from("M1m"), 2.0);
    diameters.insert(String::from("M2o"), 2.5);
    diameters.insert(String::from("M2p"), 2.3);
    diameters.insert(String::from("M2m"), 1.9);
    diameters.insert(String::from("M3o"), 2.8);
    diameters.insert(String::from("M3p"), 2.6);
    diameters.insert(String::from("M3m"), 2.3);
    diameters.insert(String::from("CPo"), 2.6);
    diameters.insert(String::from("CPp"), 2.1);
    diameters.insert(String::from("CPm"), 1.7);
    diameters.insert(String::from("CPo"), 2.6);
    diameters.insert(String::from("CIo"), 0.1);
    diameters.insert(String::from("CIp"), 0.1);
    diameters.insert(String::from("CIm"), 0.1);
    diameters.insert(String::from("CDo"), 3.2);
    diameters.insert(String::from("CDp"), 3.1);
    diameters.insert(String::from("CDm"), 2.5);
    diameters.insert(String::from("RDo"), 2.4);
    diameters.insert(String::from("RDp"), 2.3);
    diameters.insert(String::from("RDm"), 2.0);
    diameters.insert(String::from("RIo"), 2.1);
    diameters.insert(String::from("RIp"), 1.9);
    diameters.insert(String::from("RIm"), 1.5);
    diameters.insert(String::from("RPo"), 2.6);
    diameters.insert(String::from("RIp"), 2.4);
    diameters.insert(String::from("RIm"), 2.3);

    let point = point_name.trim().to_string();
    let diameter = diameters.get(&point);

    if let Some(diameter) = diameter {
        *diameter
    } else {
        1.5
    }

    // *diameter.unwrap()
} //get_diameter

/*
fn get_data_92() {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/dodge_92.csv")
        .expect("Cannot read field");

    for result in reader.records() {
        let record = result.unwrap();
        let segment_name: String = record[0].trim().to_string();

        let rho_str: String = record[5].trim().to_string();
        let theta_str: String = record[6].trim().to_string();
        let phi_str: String = record[7].trim().to_string();

        let rho: f64 = rho_str.parse().unwrap();
        let theta: f64 = theta_str.parse().unwrap();
        let phi: f64 = phi_str.parse().unwrap();

        // println!("seg:{}, size:{},  point:{}", segment_name, size, point);
        angle2pos(segment_name, rho, theta, phi);
    } //records
      // println!("vessel_vec:{:?}", vessels_vec);
      //println!("vessels_data{:?}", vessels_data);
} //read data
*/


//"SEGMENT_NAME","MEDIUM_LEN", "LONG_LEN", "RHO", "THETA",  "PHI"
/*
pub fn get_branch_data_92() {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/positions_branches.csv")
        .expect("Cannot read field");

    for result in reader.records() {
        let record = result.unwrap();
        let segment_name: String = record[0].trim().to_string();
        //let medium_diameter: f32 = record[2].parse::<f32>().unwrap();

        let rho_str: String = record[3].trim().to_string();
        let theta_str: String = record[4].trim().to_string();
        let phi_str: String = record[5].trim().to_string();

        let rho: f64 = rho_str.parse().unwrap();
        let theta: f64 = theta_str.parse().unwrap();
        let phi: f64 = phi_str.parse().unwrap();

        //"R1".to_string(),
        // println!("{:?}.to_string();", segment_name);

        //println!("diameters.insert(String::from({:?}), {:?});", segment_name, medium_diameter);
        // println!("seg:{}, size:{},  point:{}", segment_name, size, point);
        angle2pos(segment_name, rho, theta, phi);
    } //records
      // println!("vessel_vec:{:?}", vessels_vec);
      //println!("vessels_data{:?}", vessels_data);
} //read data
  /******************************************** */
*/


/*
fn angle2pos(segment: String, radius_arg: f64, theta_arg: f64, phi_arg: f64) {
    let theta = std::f64::consts::PI * theta_arg / 180.0;
    let phi = std::f64::consts::PI * phi_arg / 180.0;
    let pos_x = radius_arg * (phi.cos() * theta.sin());
    let pos_y = radius_arg * phi.sin();
    let pos_z = radius_arg * (phi.cos() * theta.cos());

    /*
    let a_segment = Segment{
        name: segment.to_string(),
        pos: vec!(pos_x, pos_y, pos_z),
        diameter: get_diameter(&segment),
    };
    */

    /*
        println!("structure segm: {:?}", a_segment);
    */

    //   println!(
    //       "seg{:?}, glVertex3f({:?},{:?},{:?});",
    //       segment, pos_x, pos_y, pos_z
    //   );

    //print!("{:?},", segment);
    //println!("{:?}, {:?},{:?},{:?};", segment, pos_x, pos_y, pos_z);
    println!(
        "points.insert(String::from({:?}), vec![{:?}, {:?}, {:?}]);",
        segment, pos_x, pos_y, pos_z
    );

    //println!("{:?}, {:?},{:?},{:?};", segment, pos_x, pos_y, pos_z);
} //angle2pos
*/
